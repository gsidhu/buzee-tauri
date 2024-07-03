import { get, readable } from 'svelte/store';
import { createTable } from 'svelte-headless-table';
import { addResizedColumns, addSortBy, addHiddenColumns } from 'svelte-headless-table/plugins';
import { documentsShown } from '$lib/stores';
import { formatUpdatedTime } from '$lib/utils/searchItemUtils';
import { readableFileSize } from '$lib/utils/miscUtils';

export function createTableFromResults(resultsShown: DocumentSearchResult[]) {
  const table = createTable(readable(resultsShown), {
    resize: addResizedColumns(),
    sort: addSortBy({ disableMultiSort: true }),
    hideCols: addHiddenColumns()
  });

  const columns = table.createColumns([
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
  ]);

  return [table, columns];
}