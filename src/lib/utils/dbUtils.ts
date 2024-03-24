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

export function categoriseExtensions(received_filetypes: FileTypes[]) {
  let allowedExtensions: FileTypesDropdown = {
		categories: [],
		items: []
	};
  received_filetypes.forEach((extension) => {
    if (allowedExtensions.categories.indexOf(extension.file_type_category) === -1) {
      allowedExtensions.categories.push(extension.file_type_category);
    }
  });
  allowedExtensions.items = received_filetypes;
  return allowedExtensions;
}

export function setExtensionCategory(extension: string | undefined, allowedExtensions: FileTypesDropdown) {
  console.log("extension:", extension);
  
  if (extension === 'any') {
    return extension;
  }
  let fileTypeString = "";
  // For each item in allowedExtensions.items, check if item.file_type_category = extension
  // If so, add it to fileTypeString
  allowedExtensions.items.forEach((item) => {
    if (item.file_type_category === extension) {
      fileTypeString += item.file_type + ",";
    }
  });
  // Remove trailing comma
  fileTypeString = fileTypeString.slice(0, -1);
  return fileTypeString;
}