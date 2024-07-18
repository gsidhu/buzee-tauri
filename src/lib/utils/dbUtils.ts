import { invoke } from "@tauri-apps/api/core";
import { extractDate, cleanSearchQuery } from "./queryParsing";
import { searchQuery, locationShown, resultsPageShown, noMoreResults, searchInProgress, filetypeShown, resultsPerPage, documentsShown, allowedExtensions, base64Images, showIconGrid,  dateLimitUNIX } from "$lib/stores";
import { trackEvent } from "@aptabase/web";
import { setExtensionCategory } from "$lib/utils/miscUtils";
import { getResultThumbnails } from '$lib/utils/fileTable';
import { get } from 'svelte/store';

export async function getDocumentsFromDB(page:number, limit:number) {
  let filetypeToGet = get(filetypeShown);
  if (filetypeToGet !== 'any') {
    filetypeToGet = setExtensionCategory(get(filetypeShown), get(allowedExtensions));
  }

  let type: String | undefined = filetypeToGet;
  if (type === "any") type = undefined;
  
  console.log("getting documents from db of type", type);
  const results: DocumentSearchResult[] = await invoke("get_recent_docs", { page: page, limit: limit*2, fileType: type });
  return results;
}

export async function searchDocuments(query:string, page:number, limit:number, type?:string, dateLimitUNIX?: ParsedDatesUNIX | null) {
  let results: DocumentSearchResult[] = [];
  console.log("searching documents with query", query, "page", page, "limit", limit, "type", type, "dateLimitUNIX", dateLimitUNIX);

  if (get(locationShown) === "computer") {
    let dateLimit: ParsedDatesUNIX | null = null;
    if (dateLimitUNIX) { dateLimit = dateLimitUNIX; }
    let parsedDates = extractDate(query);
    console.log("parsed dates:", parsedDates);
    
    if (dateLimitUNIX && dateLimitUNIX.start !== "" && dateLimitUNIX.end !== "") {
      if (parsedDates && parsedDates.start === dateLimitUNIX.start && parsedDates.end === dateLimitUNIX.end) {
        console.log("Dates are the same");
        dateLimit = dateLimitUNIX;
        query = dateLimit.text;
      } else if (parsedDates) {
        console.log("Dates are different");
        dateLimit = parsedDates;
        query = dateLimit.text;
      }
    }
    if (dateLimit && dateLimit.text.length > 0) {
      query = dateLimit.text;
    }
    let querySegments = cleanSearchQuery(query);
    
    if (type === "any") type = undefined;

    if (query.length === 0 && !(dateLimitUNIX && dateLimitUNIX.start !== "" && dateLimitUNIX.end !== "")) {
      results = await getDocumentsFromDB(page, limit);
    } else {
      if (dateLimit && dateLimit.start !== "" && dateLimit.end !== "") {
        results = await invoke("run_search", { query: JSON.stringify(querySegments), page: page, limit: limit, fileType: type, dateLimit: dateLimit});
      } else {
        results = await invoke("run_search", { query: JSON.stringify(querySegments), page: page, limit: limit, fileType: type});
      }
    }
  } else if (get(locationShown) === "browser") {
    results = await invoke("run_browser_history_search", { userProfile: "Default", userQuery: query, limit: limit, page: page});
  }
  return results;
}

export async function triggerSearch() {
  resultsPageShown.set(0); // reset the page number on each new search
  searchInProgress.set(true);
  base64Images.set([]);
  trackEvent('search-triggered', {
    filetypeShown: get(filetypeShown),
    resultsPageShown: get(resultsPageShown)
  });
  let filetypeToGet = get(filetypeShown);
  if (filetypeToGet !== 'any') {
    filetypeToGet = setExtensionCategory(get(filetypeShown), get(allowedExtensions));
  }
  
  let result = await searchDocuments(
    get(searchQuery),
    get(resultsPageShown),
    get(resultsPerPage),
    filetypeToGet,
    get(dateLimitUNIX)
  );
  documentsShown.set(result); 
  if (get(showIconGrid)) {
    console.log(">> triggersearch");
    await getResultThumbnails(get(documentsShown));
  }
  searchInProgress.set(false);
}

export async function loadMoreResults() {
  // Same function as triggerSearch, but with a different page number and appending results
  console.log("Loading more results...");
  resultsPageShown.set(get(resultsPageShown) + 1); // increment the page number on each new search
  searchInProgress.set(true);
  trackEvent('loadMoreResults', {
    filetypeShown: get(filetypeShown),
    resultsPageShown: get(resultsPageShown)
  });
  let filetypeToGet = get(filetypeShown);
  if (filetypeToGet !== 'any') {
    filetypeToGet = setExtensionCategory(get(filetypeShown), get(allowedExtensions));
  }
  let results = await searchDocuments(
    get(searchQuery),
    get(resultsPageShown),
    get(resultsPerPage),
    filetypeToGet,
  );
  if (results.length === 0) {
    noMoreResults.set(true);
  } else {
    documentsShown.set([...get(documentsShown), ...results]);
  }
  if (get(showIconGrid)) {
    console.log(">> loadmoreresults");
    await getResultThumbnails(get(documentsShown));
  }
  searchInProgress.set(false);
}