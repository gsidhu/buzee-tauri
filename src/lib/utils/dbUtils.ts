import { invoke } from "@tauri-apps/api/core";
import { extractDate, cleanSearchQuery } from "./queryParsing";

export async function getDocumentsFromDB(page:number, limit:number, type?:string) {
  if (type === "any") type = undefined;
  console.log("getting documents from db of type", type);
  const results: DocumentSearchResult[] = await invoke("get_recent_docs", { page: page, limit: limit, fileType: type });
  return results;
}

export async function searchDocuments(query:string, page:number, limit:number, type?:string) {
  const dateLimit = extractDate(query);
  if (dateLimit) {
    query = dateLimit.text;
  }
  query = cleanSearchQuery(query);
  console.log(query);
  console.log(dateLimit);
  
  if (type === "any") type = undefined;

  let results: DocumentSearchResult[];
  if (query.length === 0 && !dateLimit) {
    results = await getDocumentsFromDB(page, limit, type);
  } else {
    results = await invoke("run_search", { query: query, page: page, limit: limit, fileType: type, dateLimit: dateLimit});
  }
  console.log("search results:", results);
  // const results: DocumentSearchResult[] = [];
  return results;
}