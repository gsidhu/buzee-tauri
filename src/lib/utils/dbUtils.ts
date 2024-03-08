import { invoke } from "@tauri-apps/api/core";
import { extractDate } from "./compromise";

export async function getDocumentsFromDB(page:number, limit:number, type?:string) {
  if (type === "any") type = undefined;
  console.log("getting documents from db of type", type);
  const results: SearchResult[] = await invoke("get_recent_docs", { page: page, limit: limit, fileType: type });
  return results;
}

export async function searchDocuments(query:string, page:number, limit:number, type?:string) {
  const dateLimit = extractDate(query);
  if (dateLimit) {
    query = dateLimit.text;
  }
  console.log(query);
  console.log(dateLimit);
  
  if (type === "any") type = undefined;
  const results: SearchResult[] = query.length === 0 
    ? await getDocumentsFromDB(page, limit, type)
    : await invoke("run_search", { query: query, page: page, limit: limit, fileType: type, dateLimit: dateLimit});
  console.log("search results:", results);
  return results;
}