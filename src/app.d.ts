// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
  namespace App {
    // interface Error {}
    // interface Locals {}
    // interface PageData {}
    // interface Platform {}
  }

  // The Window interface is where the preload script exposes its API to the renderer process.
  // Compare this to the contextBridge.exposeInMainWorld() call in src/preload.ts.
  interface Window {
    electronAPI?: Electron.IpcRenderer;
    constants?: Electron.IpcRenderer;
    dbAPI?: Electron.IpcRenderer;
    menuAPI?: Electron.IpcRenderer;
    settingsAPI?: Electron.IpcRenderer;
    navigationAPI?: Electron.IpcRenderer;
  }

  // This is where declarations for types you want to use in the main process and the renderer process go.
  type ProcessVersions = NodeJS.ProcessVersions;

  interface DropdownOrButtonItem {
    handleClick: () => void,
    icon?: string,
    id?: string,
    label: string,
    marginClass?: string,
    paddingClass?: string,
    showIcon?: boolean,
    showText?: boolean,
    title?: string,
    type?: string,
    isDisabled?: boolean
  }

  type DropdownItemsArray = DropdownOrButtonItem[];

  interface DocumentSearchResult {
    id: number,
    source_domain: string,
    created_at: number,
    name: string,
    path: string,
    size: number,
    file_type: string,
    last_modified: number,
    last_opened: number,
    last_synced: number,
    last_parsed: number,
    is_pinned: boolean,
    freceny_rank: number,
    frecency_last_accessed: number,
    comment: string | null,
  }

  interface FileTypes {
    file_type: string,
    file_type_category: string,
    file_type_allowed: boolean,
    added_by_user: boolean,
  }

  interface IgnoreListType {
    path: string,
    ignore_indexing: boolean,
    ignore_content: boolean,
    is_folder: boolean,
  }

  interface FileTypesDropdown {
    categories: string[],
    items: FileTypes[]
  }

  interface StringBooleanObject {
    [key: string]: boolean;
  }

  interface Base64ImageObject {
    path: string;
    base64: string;
  }

  interface ParsedDatesUNIX {
    "start": string,
    "end": string,
    "text": string
  }

  // type ElectronStore = import('electron-store');
  // Not sure how to export the types from the ElectronStore schema without rewriting
  // So going with `any`
  interface UserPreferences {
    [key: string]: any;
  }

  interface DBStat {
    file_type: string,
    count: number
  }

  interface DateLimit {
    start: string,
    end: string,
    text: string
  }

  type Payload = {
    message: string,
    data: string
  };

  export type DragItem =
  | string[]
  | { data: string | Record<string, string>; types: string[] };

  export type DragResult = "Dropped" | "Cancelled";

  /**
   * Logical position of the cursor.
   */
  export interface CursorPosition {
    x: Number;
    y: Number;
  }

  export interface Options {
    item: DragItem;
    icon: string;
  }

  export interface CallbackPayload {
    result: DragResult;
    cursorPos: CursorPosition;
  }
}

export {};