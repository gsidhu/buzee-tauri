import { writable } from 'svelte/store'

// 1. Get the value out of storage on load.
// let storedSearchQuery = localStorage.searchQuery;
// let storedSearchOptions = localStorage.searchOptions;
// let storedSearchTrigger = localStorage.searchTrigger;
// let storedSearchResults = localStorage.searchResults;
// let storedSearchHistory = localStorage.searchHistory;
// let storedTimeTaken = localStorage.timeTaken;
// let storedNumResultsReceived = localStorage.numResultsReceived;
// let storedResultFolderPaths = localStorage.resultFolderPaths;
// let storedResultFolderObjects = localStorage.resultFolderObjects;
let storedCompactViewMode = false;
if (typeof window !== "undefined") {
  storedCompactViewMode = localStorage.compactViewMode === 'true';
}

// 1.5 Not storing values in localStorage yet because then the last search shows up on each new load.
let storedSearchQuery = '';
let storedSearchOptions: DropdownItemsArray = [
  {
    label: "Exact Search",
    handleClick: () => {
      console.log("Exact search");
    },
  },
];
let storedDocumentsShown: SearchResult[] = [];
// let storedSearchTrigger = false;
// let storedSearchResults = [];
// let storedSearchHistory = [];
// let storedTimeTaken = 0;
// let storedNumResultsReceived = 0;
// let storedDBStats = {};
// let storedCurrentCollection = 'Everything';
// let currentUser = null;
let storedSelectedResult: SearchResult = {name: '', path: ''};

// 2. Set the stored value or a sane default.
export const pinMode = writable(false)
export const searchQuery = writable(storedSearchQuery || '')
export const searchOptions = writable(storedSearchOptions || '')
export const documentsShown = writable(storedDocumentsShown || [])
export const filetypeShown = writable('.any')
export const resultsPageShown = writable(0)
export const resultsPerPage = writable(50)
export const promptUser = writable("")
export const compactViewMode = writable(storedCompactViewMode || false)
export const selectedResult = writable(storedSelectedResult || {})
// export const searchTrigger = writable(storedSearchTrigger || false)
// export const searchResults = writable(storedSearchResults || '')
// export const searchHistory = writable(storedSearchHistory || '')
// export const timeTaken = writable(storedTimeTaken || 0)
// export const numResultsReceived = writable(storedNumResultsReceived || 0)
// export const dbStats = writable(storedDBStats || '')
// export const deleteFile = writable('')
// export const currentCollection = writable(storedCurrentCollection || '')
// export const user = writable(currentUser || null);
export const shiftKeyPressed = writable(false);
export const metaKeyPressed = writable(false);
export const mouseDown = writable(false);
export const searchInProgress = writable(false);
export const dbCreationInProgress = writable(false);

// 3. Anytime the store changes, update the local storage value.
if(typeof window !== "undefined") {
  pinMode.subscribe(value => { localStorage.pinMode = value })
  searchQuery.subscribe(value => { localStorage.searchQuery = value })
  searchOptions.subscribe(value => { localStorage.searchOptions = value })
  documentsShown.subscribe(value => { localStorage.documentsShown = value })
  filetypeShown.subscribe(value => { localStorage.filetypeShown = value })
  resultsPageShown.subscribe(value => { localStorage.resultsPageShown = value })
  resultsPerPage.subscribe(value => { localStorage.resultsPerPage = value })
  promptUser.subscribe(value => { localStorage.promptUser = value })
  compactViewMode.subscribe(value => { localStorage.compactViewMode = value })
  // searchTrigger.subscribe(value => { localStorage.searchTrigger = value })
  // searchResults.subscribe(value => { localStorage.searchResults = value })
  // searchHistory.subscribe(value => { localStorage.searchHistory = value })
  // timeTaken.subscribe(value => { localStorage.timeTaken = value })
  // numResultsReceived.subscribe(value => { localStorage.numResultsReceived = value })
  // dbStats.subscribe(value => { localStorage.dbStats = value })
  // currentCollection.subscribe(value => { localStorage.currentCollection = value })
  // user.subscribe(value => { localStorage.user = value })
}
