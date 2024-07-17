import { get, readable, writable } from 'svelte/store';
import { createTable } from 'svelte-headless-table';
import { addResizedColumns, addSortBy, addHiddenColumns, addPagination } from 'svelte-headless-table/plugins';
import { documentsShown, resultsPerPage, base64Images, locationShown } from '$lib/stores';
import { formatUpdatedTime } from '$lib/utils/searchItemUtils';
import { readableFileSize } from '$lib/utils/miscUtils';
import { invoke } from "@tauri-apps/api/core";

export function createTableFromResults(resultsShown: DocumentSearchResult[]) {
  const table = createTable(readable(resultsShown), {
    resize: addResizedColumns(),
    sort: addSortBy({ disableMultiSort: true }),
    hideCols: addHiddenColumns(),
    page: addPagination({ initialPageSize: get(resultsPerPage) }),
  });

  let columnsArray:any = [];
  if (get(locationShown) === 'computer') {
    columnsArray = [
      table.column({
        header: 'Type',
        accessor: 'file_type',
        plugins: {
          resize: {
            initialWidth: 30,
            minWidth: 30,
            maxWidth: 30
          },
          sort: { disable: false }
        }
      }),
      table.column({
        header: 'Name',
        accessor: 'name',
        plugins: {
          resize: {
            initialWidth: 250,
            minWidth: 250,
            maxWidth: 250
          }
        }
      }),
      table.column({
        header: 'Last Modified',
        accessor: 'last_modified',
        id: 'lastModified',
        cell: ({ value }: { value: number }) => formatUpdatedTime(value) ?? value,
        plugins: {
          resize: {
            initialWidth: 140,
            minWidth: 125,
            maxWidth: 150
          }
        }
      }),
      table.column({
        header: 'Last Opened',
        accessor: 'last_opened',
        id: 'lastOpened',
        cell: ({ value }: { value: number }) => formatUpdatedTime(value) ?? value,
        plugins: {
          resize: {
            initialWidth: 140,
            minWidth: 125,
            maxWidth: 150
          }
        }
      }),
      table.column({
        header: 'Size',
        accessor: 'size',
        id: 'size',
        cell: ({ value }: { value: number }) => readableFileSize(value) ?? "",
        plugins: {
          resize: {
            initialWidth: 75,
            minWidth: 75,
            maxWidth: 75
          }
        }
      }),
      table.column({
        header: 'Location',
        accessor: 'path',
        plugins: {
          resize: {
            initialWidth: 200,
            minWidth: 200,
            maxWidth: 200
          }
        }
      })
    ];
  } else if (get(locationShown) === 'browser') {
    columnsArray = [
      table.column({
        header: 'Type',
        accessor: 'file_type',
        plugins: {
          resize: {
            initialWidth: 30,
            minWidth: 30,
            maxWidth: 30
          },
          sort: { disable: false }
        }
      }),
      table.column({
        header: 'Title',
        accessor: 'name',
        plugins: {
          resize: {
            initialWidth: 250,
            minWidth: 250,
            maxWidth: 250
          }
        }
      }),
      table.column({
        header: 'Last Opened',
        accessor: 'last_opened',
        id: 'lastOpened',
        cell: ({ value }: { value: number }) => formatUpdatedTime(value) ?? value,
        plugins: {
          resize: {
            initialWidth: 140,
            minWidth: 125,
            maxWidth: 150
          }
        }
      }),
      table.column({
        header: 'URL',
        accessor: 'path',
        plugins: {
          resize: {
            initialWidth: 250,
            minWidth: 250,
            maxWidth: 250
          }
        }
      }),
    ];
  }

  // @ts-ignore
  const columns = table.createColumns(columnsArray);

  return [table, columns];
}

export async function getResultThumbnails(resultsShown: DocumentSearchResult[]) {
  console.log(`Getting thumbnails for ${resultsShown.length} results`);
  
  resultsShown.forEach(async (result) => {
    if (['png', 'jpg', 'jpeg', 'gif', 'svg', 'webp'].includes(result.file_type) && findBase64ImageObjectFromPath(result.path).base64 === '') {
      console.log(`Getting base64 image for ${result.name}`);
      await invoke('get_image_base64', { filePath: result.path }).then((res) => {
        let newBase64Images = get(base64Images);
        // @ts-ignore
        newBase64Images.push({ path: result.path, base64: res });
        base64Images.set(newBase64Images);
      })
    }
  });

  console.log(get(base64Images));
}

export function findBase64ImageObjectFromPath(path: string) {
  // $base64Images is an array of objects with keys path and base64
  // find the index of the element in base64Images that has the same path as the path argument
  let index = -1;
  for (let i = 0; i < get(base64Images).length; i++) {
    if (get(base64Images)[i].path === path) {
      index = i;
    }
  }
  console.log(">>>", path, index);
  if (index !== -1) {
    return get(base64Images)[index];
  } else {
    return { path: path, base64: '' };
  }
}