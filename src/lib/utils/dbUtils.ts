import { invoke } from "@tauri-apps/api/core";
import { extractDate, cleanSearchQuery } from "./queryParsing";

export async function getDocumentsFromDB(page:number, limit:number, type?:string) {
  if (type === "any") type = undefined;
  console.log("getting documents from db of type", type);
  const results: DocumentSearchResult[] = await invoke("get_recent_docs", { page: page, limit: limit*2, fileType: type });
  return results;
}

export async function searchDocuments(query:string, page:number, limit:number, type?:string, dateLimitUNIX?: ParsedDatesUNIX | null) {
  console.log("searching documents with query", query, "page", page, "limit", limit, "type", type, "dateLimitUNIX", dateLimitUNIX);
  let dateLimit: ParsedDatesUNIX | null;
  if (dateLimitUNIX) {
    dateLimit = dateLimitUNIX;
  } else {
    dateLimit = extractDate(query);
  }
  if (dateLimit) {
    query = dateLimit.text;
  }
  let querySegments = cleanSearchQuery(query);
  
  if (type === "any") type = undefined;

  let results: DocumentSearchResult[] = [];
  if (query.length === 0 && !dateLimit) {
    results = await getDocumentsFromDB(page, limit, type);
  } else {
    results = await invoke("run_search", { query: JSON.stringify(querySegments), page: page, limit: limit, fileType: type, dateLimit: dateLimit});
  }
  return results;
}