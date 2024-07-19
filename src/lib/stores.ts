import { writable } from 'svelte/store'

// 1. Get the value out of storage on load.
// let storedSearchQuery = localStorage.searchQuery;
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
let storedDocumentsShown: DocumentSearchResult[] = [];
// let storedSearchTrigger = false;
// let storedSearchResults = [];
// let storedSearchHistory = [];
// let storedTimeTaken = 0;
// let storedNumResultsReceived = 0;
// let storedDBStats = {};
// let storedCurrentCollection = 'Everything';
// let currentUser = null;
let storedSelectedResult: DocumentSearchResult = {
  id: 0,
  source_domain: '',
  created_at: 0,
  name: '',
  path: '',
  size: 0,
  file_type: '',
  last_modified: 0,
  last_opened: 0,
  last_synced: 0,
  last_parsed: 0,
  is_pinned: false,
  freceny_rank: 0,
  frecency_last_accessed: 0,
  comment: null,
};

let storedAllowedExtensions: FileTypesDropdown = {
  categories: [],
  items: []
};

let storedAllowedLocations = ["computer", "browser"];

let storedDateLimitUNIX: ParsedDatesUNIX = {
  start: '',
  end: '',
  text: ''
};

let storedFileText: string[] = [];
let storedSearchSuggestions: string[] = [];
let storedIgnoredPaths: IgnoreListType[] = [];
let storedBase64Images: Base64ImageObject[] = [];

// 2. Set the stored value or a sane default.
export const userPreferences = writable({
  "automatic_background_sync": true,
  "detailed_scan": true,
  "first_launch_done": true,
  "global_shortcut": "Alt+Space",
  "global_shortcut_enabled": true,
  "show_search_suggestions": true,
  "launch_at_startup": true,
  "onboarding_done": false,
  "show_in_dock": true,
  "roadmap_survey_answered": false,
  "skip_parsing_pdfs": true,
  "manual_setup": false,
})
export const pagePath = writable("")
export const isMac = writable(false)
export const pinMode = writable(false)
export const showIconGrid = writable(false)
export const cronJobSet = writable(false)
export const onSearchPage = writable(false)
export const onboardingDone = writable(false)
export const syncStatus = writable(false)
export const disableInteraction = writable(false)
export const searchQuery = writable(storedSearchQuery || '')
export const searchSuggestions = writable(storedSearchSuggestions || [])
export const documentsShown = writable(storedDocumentsShown || [])
export const filetypeShown = writable('any')
export const locationShown = writable('computer')
export const allowedExtensions = writable(storedAllowedExtensions);
export const allowedLocations = writable(storedAllowedLocations);
export const resultsPageShown = writable(0)
export const resultsPerPage = writable(25)
export const statusMessage = writable("")
export const compactViewMode = writable(storedCompactViewMode || false)
export const selectedResult = writable(storedSelectedResult || {})
export const selectedResultText = writable(storedFileText || [])
export const ignoredPaths = writable(storedIgnoredPaths || [])
export const shiftKeyPressed = writable(false);
export const metaKeyPressed = writable(false);
export const mouseDown = writable(false);
export const searchInProgress = writable(false);
export const base64SearchInProgress = writable(false);
export const dbCreationInProgress = writable(false);
export const windowBlurred = writable(false);
export const scratchPadText = writable("")
export const base64Images = writable(storedBase64Images || [])
export const preferLastOpened = writable(false);
export const showResultTextPreview = writable(false);
export const noMoreResults = writable(false);
export const searchSuggestionsDialogOpen = writable(false);
export const searchFiltersOpen = writable(false);
export const ignoreDialogOpen = writable(false);
export const dateLimitUNIX = writable(storedDateLimitUNIX || null)

// 3. Anytime the store changes, update the local storage value.
if(typeof window !== "undefined") {
  pinMode.subscribe(value => { localStorage.pinMode = value })
  searchQuery.subscribe(value => { localStorage.searchQuery = value })
  documentsShown.subscribe(value => { localStorage.documentsShown = value })
  filetypeShown.subscribe(value => { localStorage.filetypeShown = value })
  resultsPageShown.subscribe(value => { localStorage.resultsPageShown = value })
  resultsPerPage.subscribe(value => { localStorage.resultsPerPage = value })
  statusMessage.subscribe(value => { localStorage.statusMessage = value })
  compactViewMode.subscribe(value => { localStorage.compactViewMode = value })
  scratchPadText.subscribe(value => { localStorage.scratchPadText = value })
  // searchTrigger.subscribe(value => { localStorage.searchTrigger = value })
  // searchResults.subscribe(value => { localStorage.searchResults = value })
  // searchHistory.subscribe(value => { localStorage.searchHistory = value })
  // timeTaken.subscribe(value => { localStorage.timeTaken = value })
  // numResultsReceived.subscribe(value => { localStorage.numResultsReceived = value })
  // dbStats.subscribe(value => { localStorage.dbStats = value })
  // currentCollection.subscribe(value => { localStorage.currentCollection = value })
  // user.subscribe(value => { localStorage.user = value })
}
