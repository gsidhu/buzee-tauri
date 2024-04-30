<script lang="ts">
	import { invoke, transformCallback } from "@tauri-apps/api/core";
	import moment from 'moment';
	import { onMount } from 'svelte';
	import { readable } from 'svelte/store';
	import { documentsShown, shiftKeyPressed, compactViewMode, selectedResult, resultsPageShown, searchInProgress, filetypeShown, allowedExtensions, searchQuery, resultsPerPage } from '$lib/stores';
	import { searchDocuments } from '$lib/utils/dbUtils';
	import { setExtensionCategory } from '$lib/utils/miscUtils';
	import FileTypeIcon from '$lib/components/ui/FileTypeIcon.svelte';
	import FiletypeDropdown from '$lib/components/search/FiletypeDropdown.svelte';
	// @ts-ignore
	import { createTable, Subscribe, Render } from 'svelte-headless-table';
	// @ts-ignore
	import { addResizedColumns, addSortBy, addHiddenColumns } from 'svelte-headless-table/plugins';
	import { stringToHash, readableFileSize, resetColumnSize } from '$lib/utils/miscUtils';
	import { clickRow } from '$lib/utils/fileUtils';
	import { sendEventToFirebase } from '../../../utils/firebase';
	import ConfettiButton from '../ui/confettiButton.svelte';
	import { goto } from "$app/navigation";

	let noMoreResults = false;

	$: if ($documentsShown.length < 50) {
		noMoreResults = true;
	}

	async function loadMoreResults() {
		// Same function as triggerSearch, but with a different page number and appending results
		console.log("Loading more results...");
		$resultsPageShown += 1; // increment the page number on each new search
		$searchInProgress = true;
		sendEventToFirebase('search-triggered', {
			searchQuery: $searchQuery,
			filetypeShown: $filetypeShown,
			resultsPageShown: $resultsPageShown
		});
		let filetypeToGet = $filetypeShown;
		if (filetypeToGet !== 'any') {
			filetypeToGet = setExtensionCategory($filetypeShown, $allowedExtensions);
		}
		let results = await searchDocuments(
			$searchQuery,
			$resultsPageShown,
			$resultsPerPage,
			filetypeToGet
		);
		sendEventToFirebase('search-results', { searchQuery: $searchQuery, resultsLength: results?.length });
		if (results.length === 0) {
			noMoreResults = true;
		} else {
			$documentsShown = [...$documentsShown, ...results];
		}
		$searchInProgress = false;
	}
  
  function startDragging(filepath: string) {
		const image64 = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAAABHNCSVQICAgIfAhkiAAAAAlwSFlzAAAA7AAAAOwBeShxvQAAABl0RVh0U29mdHdhcmUAd3d3Lmlua3NjYXBlLm9yZ5vuPBoAAADySURBVFiF7dcxSgNRFIXhT4Wx1NrCFCK4iEiCWxBcgjsQscteLMQlKCksLCLYWbkE3cEEJBaTgL5MzGReAhb3h9cc3tz7w0wxh3k6GKLEpOV5R7dmdiOGuEHR8vkBHvGBfpsBZcbymcBgunypxHZNVmCcITDjCee4x9kqAuvkeSpx95dEyiRz6SVuk6yPT5yml7cWCNTlTdnHK0Z4+5F3cYyTTQvAHi5wlOTXTWbnvoKVZm/6I1xKCIRACIRACIRACIRACPxLgbG8araIXVXt+8VOzcUeDvCCrzUtL3Cl+iVPS8scHVW7zann6SnxgMN02Ter0UNOfhP2XAAAAABJRU5ErkJggg==";
    startDrag({ item: [filepath], icon: image64 })
  }

  async function startDrag(
    options: Options,
    onEvent?: (result: CallbackPayload) => void
  ): Promise<void> {
    await invoke("start_drag", {
      item: options.item,
      image: options.icon,
      onEventFn: onEvent ? transformCallback(onEvent) : null,
    });
  }

	function openFile(url: string) {
		sendEventToFirebase('click:open_file');
		invoke('open_file_or_folder', { filePath: url });
	}
	
	function formatUpdatedTime(unixTime: number): string {
		if (unixTime === 0) {
			return 'Never';
		}
		let unixToJs = new Date(unixTime*1000);
		const updatedMoment = moment(unixToJs);
		const today = moment();
		const yesterday = moment().subtract(1, 'days');

		if (updatedMoment.isSame(today, 'day')) {
			// If the update was today, return the time
			return updatedMoment.format('h:mm A');
		} else if (updatedMoment.isSame(yesterday, 'day')) {
			// If the update was yesterday, return 'Yesterday'
			return 'Yesterday';
		} else {
			// Otherwise, return the date
			return updatedMoment.format('YYYY-MM-DD');
		}
	}

	function formatPath(url: string): string {
		const parts = url.split('/'); // Split the url into components
		const length = parts.length;

		if (length > 3) {
			// Take the two directories just before the file name and prepend '...'
			return '.../' + parts.slice(length - 3, length - 1).join('/') + '/';
		} else {
			// If the url is already short, return it as is
			return url;
		}
	}

	async function showContextMenu(
		e: MouseEvent & { currentTarget: EventTarget & HTMLDivElement },
		result: DocumentSearchResult
	) {
		sendEventToFirebase('right_click:result_context_menu');
		clickRow(e, $shiftKeyPressed);
		// window.menuAPI?.showResultsContextMenu(result);
		$selectedResult = result;
		await invoke("open_context_menu", {option:"searchresult"});
	}

	const table = createTable(readable($documentsShown), {
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
					initialWidth: 20,
					minWidth: 20,
					maxWidth: 20
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
					minWidth: 150,
					maxWidth: 500
				}
			}
		}),
		table.column({
			header: 'Last Modifed',
			accessor: 'last_modified',
			id: 'lastModified',
			cell: ({ value }: { value: number }) => formatUpdatedTime(value) ?? value,
			plugins: {
				resize: {
					initialWidth: 100,
					minWidth: 100,
					maxWidth: 100
				}
			}
		}),
		// table.column({
		// 	header: 'Last Opened',
		// 	accessor: 'frecency_last_accessed',
		// 	id: 'lastOpened',
		// 	cell: ({ value }: { value: number }) => formatUpdatedTime(value) ?? value,
		// 	plugins: {
		// 		resize: {
		// 			initialWidth: 100,
		// 			minWidth: 100,
		// 			maxWidth: 100
		// 		}
		// 	}
		// }),
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
					minWidth: 50,
					maxWidth: 500
				}
			}
		})
	]);

	function showTableHeaderContextMenu(option: string) {
		window.menuAPI?.showTableHeaderMenu(option, ids, labels, hideForId);
	}

	const { flatColumns, headerRows, rows, tableAttrs, tableBodyAttrs, pluginStates } =
		table.createViewModel(columns);
	const { hiddenColumnIds } = pluginStates.hideCols;
	const ids = flatColumns.map((c: any) => c.id);
	const labels = flatColumns.map((c: any) => c.header);
	let hideForId: Record<string, boolean> = Object.fromEntries(ids.map((id: any) => [id, false]));
	$: $hiddenColumnIds = Object.entries(hideForId)
		.filter(([, hide]) => hide)
		.map(([id]) => id);

	// HACK: hide size column by default
	// hideForId['size'] = true;
	// hideForId['lastOpened'] = true;

	onMount(() => {
		// select the first result when loading new search results
		$selectedResult = $documentsShown[0];
		let firstResult = document.querySelector('.result-0') as HTMLElement | null;
		if (firstResult) {
			firstResult.focus();
		}

		resetColumnSize();
		// window.electronAPI?.resetTableColWidths(() => {
		// 	resetColumnSize();
		// });
		// window.electronAPI?.showHideColumn((id: string, hide: boolean) => {
		// 	let falseCount = Object.values(hideForId).filter((value) => value === false).length;
		// 	if (falseCount <= 2 && hide) {
		// 		return;
		// 	}
		// 	console.log(id, hide);

		// 	hideForId[id] = hide;
		// 	// HACK: if no columns are hidden after this, reset the column size
		// 	falseCount = Object.values(hideForId).filter((value) => value === false).length;
		// 	if (falseCount === ids.length) {
		// 		// window.location.reload();
		// 		resetColumnSize();
		// 	}
		// });
	});
</script>

<table {...$tableAttrs}>
	<thead id="real-thead">
		{#each $headerRows as headerRow (headerRow.id)}
			<Subscribe rowAttrs={headerRow.attrs()} let:rowAttrs>
				<tr {...rowAttrs}>
					{#each headerRow.cells as cell (cell.id)}
						<Subscribe attrs={cell.attrs()} let:attrs props={cell.props()} let:props>
							<th
								{...attrs}
								class={`${cell.id}-col ${$compactViewMode ? 'compact-view' : ''}`}
								role="button"
								tabindex="0"
								use:props.resize
								on:click={props.sort.toggle}
								class:sorted={props.sort.order !== undefined}
								on:contextmenu={() => showTableHeaderContextMenu(cell.id)}
							>
								{#if cell.id === 'file_type'}
									<!-- <FiletypeDropdown searchBar={false} /> -->
									<FileTypeIcon filetype="other" />
								{:else}
									<Render of={cell.render()} />
								{/if}
								{#if props.sort.order === 'asc'}
									<i class="bi bi-caret-up-fill" />
								{:else if props.sort.order === 'desc'}
									<i class="bi bi-caret-down-fill" />
								{/if}
								{#if !props.resize.disabled}
									<button
										aria-hidden="false"
										tabindex="-1"
										class="resizer"
										on:click|stopPropagation
										use:props.resize.drag
										use:props.resize.reset
									/>
								{/if}
							</th>
						</Subscribe>
					{/each}
				</tr>
			</Subscribe>
		{/each}
	</thead>
	<!-- {#key $documentsShown.length} -->
		{#if $documentsShown.length > 0}
			<tbody {...$tableBodyAttrs}>
				{#each $rows as row (row.id)}
					<Subscribe rowAttrs={row.attrs()} let:rowAttrs>
						<tr
							{...rowAttrs}
							id={stringToHash($documentsShown[Number(row.id)].path)}
							class={`table-row result-${Number(row.id)}`}
							role="button"
							tabindex="0"
							on:focus={(e) => clickRow(e, $shiftKeyPressed)}
							on:click={(e) => clickRow(e, $shiftKeyPressed)}
							on:contextmenu={(e) => showContextMenu(e, $documentsShown[Number(row.id)])}
							on:dblclick={() => openFile($documentsShown[Number(row.id)].path)}
							draggable="true"
							on:dragstart={(event) => startDragging($documentsShown[Number(row.id)].path)}
						>
							{#each row.cells as cell (cell.id)}
								<Subscribe attrs={cell.attrs()} let:attrs>
									<td {...attrs} class={`${cell.id}-col ${$compactViewMode ? 'compact-view' : ''}`}
										title={cell.id === 'name' || cell.id === 'path' ? String(cell.render()) : ''}
									>
										{#if cell.id === 'file_type'}
											<FileTypeIcon filetype={String(cell.render())} />
										{:else if cell.id === 'name'}
											{#if $documentsShown[Number(row.id)].last_parsed > 0}
												<span class="d-flex align-items-center gap-1">
													<i class="bi bi-check-circle fs-small" title="Item contents scanned" style="font-size: 8px; color: var(--bs-success);"></i>
													<Render of={cell.render()} />
												</span>
											{:else}
												<span><Render of={cell.render()} /></span>
											{/if}
										{:else}
											<span><Render of={cell.render()} /></span>
										{/if}
									</td>
								</Subscribe>
							{/each}
						</tr>
					</Subscribe>
				{/each}
				{#if !noMoreResults}
					<tr class="table-row text-center" draggable="false">
						<td>
							<ConfettiButton 
								label="Load more"
								type="confetti-button btn-sm"
								showText={!$searchInProgress}
								showSpinner={$searchInProgress}
								handleClick={() => loadMoreResults()} />
						</td>
					</tr>
				{/if}
			</tbody>
		{/if}
	<!-- {/key} -->
</table>

{#if $documentsShown.length <= 0}
	<div class="d-flex flex-column px-4 py-2 mx-auto align-items-center justify-content-center min-vh-80">
		<img id="buzee-logo-img" class="w-25 my-2" src="/Buzee Logo.png" alt="No Results" />
		<h3>No Results</h3>
		<div class="d-flex flex-column text-light-emphasis text-center small gap-2">
			<span>Try modifying your query? You can be more specific like –</span>
			<span><code>last year "annual report" -pdf</code></span>
		</div>
		<button type="button" class="my-2 btn btn-sm purple border-hover-purple border-2 border-light rounded" on:click={() => goto('/magic/tips-shortcuts')}>View all tips and shortcuts</button>
	</div>
{/if}

<style lang="scss">
	.min-vh-80 {
		min-height: 80vh !important;
	}
	#buzee-logo-img {
		max-width: 200px;
	}
	table {
		border-spacing: 0;
		width: 100%;
		position: relative;
	}
	tr {
		cursor: default;
	}
	td {
		position: relative;
		overflow: hidden;
		span {
			display: block;
			width: 100%;
			overflow: hidden;
			text-overflow: ellipsis;
			white-space: nowrap;
		}
	}
	// regular padding
	td {
		font-size: 0.9rem;
		padding: 8px;
	}
	// compact padding
	td.compact-view,
	th.compact-view {
		font-size: 0.8rem;
		padding: 4px !important;
	}
	th {
		// border-bottom: 1px solid var(--light-purple);
		text-align: center;
		font-size: 0.9rem !important;
		font-weight: 600;
		padding: 4px 4px !important;
	}
	th:last-of-type {
		overflow-x: clip;
	}
	.type-col,
	.size-col,
	.lastModified-col,
	.lastOpened-col {
		text-align: center;
	}
	// banded rows
	tr:not(.selected):nth-of-type(2n + 1) > td {
		background-color: #d3d3d340;
	}
	// selected row
	.selected {
		background-color: var(--purple) !important;
		color: white;
		.pinned,
		.pin {
			color: white;
		}
		&.grayscale {
			filter: grayscale(.7);
			background-color: transparent !important;
		}
	}
	// pinned rows
	.pinned {
		color: var(--hot-pink);
	}
	.pin {
		color: var(--bs-body-color);
	}
	.pin:hover {
		color: var(--hot-pink);
	}
	// resize column handle
	th {
		position: relative; // need this to position the resizer
	}
	th .resizer {
		position: absolute;
		top: 0%;
		bottom: 0;
		right: -4px;
		width: 4px;
		height: 100%;
		background: black;
		// background: transparent;
		cursor: col-resize;
		z-index: 1;
		opacity: 0.05;

		&:hover {
			top: 0;
			height: 100%;
			width: 8px;
			background: var(--purple);
			opacity: 1;
		}
	}
	button.resizer {
		border: none;
		padding: 0;
	}

	// table head fixed
	thead,
	tbody tr {
		display: table;
		width: 100%;
		table-layout: fixed; /* even columns width , fix width of table too*/
	}

	tbody {
		display: block;
		overflow-y: scroll;
		overflow-x: auto !important;
		max-height: calc(
			100vh - 110px
		); /* set maximum height so rows don't hide outside the viewport; 110px is roughly the height of the topbar + thead + statusbar + loadMore button */
	}
</style>
