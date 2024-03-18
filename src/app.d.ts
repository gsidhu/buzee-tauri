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

  interface SearchResult {
    // id: number,
    name: string,
    path: string,
    // size: number,
    // file_type: string,
    // last_modified: Date,
    // pinned: boolean,
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
}

export {};