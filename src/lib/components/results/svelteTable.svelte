<script lang="ts">
	import { onMount, tick } from 'svelte';
	import { documentsShown, preferLastOpened, shiftKeyPressed, compactViewMode, selectedResult, showResultTextPreview, noMoreResults, searchInProgress, showIconGrid, base64Images } from '$lib/stores';
	import FileTypeIcon from '$lib/components/ui/FileTypeIcon.svelte';
	import { stringToHash, resetColumnSize } from '$lib/utils/miscUtils';
	import { clickRow } from '$lib/utils/fileUtils';
	import { trackEvent } from '@aptabase/web';
	import { Button } from "$lib/components/ui/button";
	import * as ContextMenu from "$lib/components/ui/context-menu";
	import ResultTextPreview from "./ResultTextPreview.svelte";
	import { openFileFolder, openFile, formatPath, startDragging } from '$lib/utils/searchItemUtils';
	import { createTableFromResults, getResultThumbnails, findBase64ImageObjectFromPath } from '$lib/utils/fileTable';
	// @ts-ignore
	import { Subscribe, Render } from 'svelte-headless-table';
	import Label from '../ui/label/label.svelte';
	import { loadMoreResults } from '$lib/utils/dbUtils';
	import { Check, LoaderCircle } from 'lucide-svelte';

	function showHideColumn(colID: string) {
		console.log("Hiding column", colID);
		trackEvent('right_click:resultTableHeaderContextMenu', {colID});
		resetColumnSize();
		hideForId[colID] = !hideForId[colID];
		if (colID === 'lastModified' || colID === 'lastOpened') {
			if (hideForId['lastModified']) {
				$preferLastOpened = true;
			}
		}
	}

	function createTableVars(dataRows: DocumentSearchResult[]) {
		const [table, columns] = createTableFromResults(dataRows);
		// @ts-ignore
		const { flatColumns, headerRows, pageRows, rows, tableAttrs, tableBodyAttrs, pluginStates } = table.createViewModel(columns);
		const { hasNextPage, hasPreviousPage, pageIndex, pageCount, pageSize } = pluginStates.page;
		const { hiddenColumnIds } = pluginStates.hideCols;
		const ids = flatColumns.map((c: any) => c.id);
		const labels = flatColumns.map((c: any) => c.header);
		const hideForId: Record<string, boolean> = Object.fromEntries(ids.map((id: any) => [id, false]));
		return { table, columns, flatColumns, headerRows, pageRows, rows, tableAttrs, tableBodyAttrs, pluginStates, hasNextPage, hasPreviousPage, pageIndex, pageCount, pageSize, hiddenColumnIds, ids, labels, hideForId };
	}

	let { table, columns, flatColumns, headerRows, pageRows, rows, tableAttrs, tableBodyAttrs, pluginStates, hasNextPage, hasPreviousPage, pageIndex, pageCount, pageSize, hiddenColumnIds, ids, labels, hideForId } = createTableVars($documentsShown);
	// HACK: hide columns by default
	// hideForId['size'] = true;
	if ($preferLastOpened) {
		hideForId['lastModified'] = true;
	} else {
		hideForId['lastOpened'] = true;
	}
	// @ts-ignore
	let columnsArray = columns.map((column: any) => ({ id: column.id, header: column.header }));

	$: if ($documentsShown) {
		console.log(">>> reloading...");
		
		({ table, columns, flatColumns, headerRows, pageRows, rows, tableAttrs, tableBodyAttrs, pluginStates, hasNextPage, hasPreviousPage, pageIndex, pageCount, pageSize, hiddenColumnIds, ids, labels, hideForId } = createTableVars($documentsShown));
		
		if ($preferLastOpened) {
			hideForId['lastModified'] = true;
		} else {
			hideForId['lastOpened'] = true;
		}
		// @ts-ignore
		columnsArray = columns.map((column: any) => ({ id: column.id, header: column.header }));
	}
	
	$: $hiddenColumnIds = Object.entries(hideForId)
		.filter(([, hide]) => hide)
		.map(([id]) => id);

	function findBase64ImageObjectFromPathLocal(path: string) {
		let imageObject = $base64Images.find(image => image.path === path);
		console.log(">> imageObject?", imageObject);
		if (imageObject) {
			return imageObject;
		} else {
			return { path: '', base64: '' };
		}
	}

	onMount(async () => {
		// select the first result when loading new search results
		$selectedResult = $documentsShown[0];
		let firstResult = document.querySelector('.result-0') as HTMLElement | null;
		if (firstResult) {
			firstResult.focus();
		}
		resetColumnSize();

		// always get thumbnails when the table is loaded for the first time
		console.log(">> sveltetable mount");
		
		getResultThumbnails($documentsShown);
  })
</script>

{#if $showIconGrid}
	<div id="parent-grid" class="flex flex-col">
		<div class={`file-grid p-2 ${$compactViewMode ? 'gap-2' : 'gap-4'}`}>
			{#each $pageRows as row (row.id)}
				<ContextMenu.Root>
					<ContextMenu.Trigger>
					<button
						id={stringToHash($documentsShown[Number(row.id)].path)}
						style="all: unset;"
						class={`icon-item w-full h-full p-1 grid items-center justify-between table-row result-${Number(row.id)} ${$compactViewMode ? 'compact-view' : ''}`}
						tabindex="0"
						on:focus={(e) => clickRow(e, $shiftKeyPressed)}
						on:click={(e) => clickRow(e, $shiftKeyPressed)}
						on:dblclick={() => openFile($documentsShown[Number(row.id)].path)}
						draggable="true"
						on:dragstart={(event) => startDragging($documentsShown[Number(row.id)].path)}
						title={$documentsShown[Number(row.id)].name}
					>
						<div class="flex justify-center">
							{#if ['png', 'jpg', 'jpeg', 'gif', 'svg', 'webp'].includes($documentsShown[Number(row.id)].file_type)}
								{#if $searchInProgress}
									<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
								{:else}
									<img src={"data:image/png;base64, " + findBase64ImageObjectFromPathLocal($documentsShown[Number(row.id)].path).base64} alt={$documentsShown[Number(row.id)].name} class={`img-thumbnail ${$compactViewMode ? 'compact-view' : ''}`} />
								{/if}
							{:else}
								<FileTypeIcon filetype={$documentsShown[Number(row.id)].file_type} extraClasses={`${$compactViewMode ? 'text-lg' : 'text-2xl'}`}/>
							{/if}
						</div>
						<div class="filename text-center p-1 w-full">
							{$documentsShown[Number(row.id)].name}
						</div>
					</button>
				</ContextMenu.Trigger>
				<ContextMenu.Content>
					{#if $documentsShown[Number(row.id)].file_type !== 'folder' && $documentsShown[Number(row.id)].last_parsed !== 0}
						<ContextMenu.Item on:click={() => {$showResultTextPreview = true; $selectedResult = $documentsShown[Number(row.id)];}}>
							Show Preview
						</ContextMenu.Item>
					{/if}
					<ContextMenu.Item>
						Open {$documentsShown[Number(row.id)].file_type === 'folder' ? 'Folder' : 'File'}
					</ContextMenu.Item>
					<ContextMenu.Sub>
						<ContextMenu.SubTrigger>Ignore</ContextMenu.SubTrigger>
						<ContextMenu.SubContent class="w-48">
							<ContextMenu.Item>Ignore this {$documentsShown[Number(row.id)].file_type === 'folder' ? 'folder' : 'file'}</ContextMenu.Item>
							<ContextMenu.Item>Ignore parent folder</ContextMenu.Item>
							{#if $documentsShown[Number(row.id)].file_type !== 'folder'}
								<ContextMenu.Item>Ignore this file's text</ContextMenu.Item>
							{/if}
						</ContextMenu.SubContent>
					</ContextMenu.Sub>
				</ContextMenu.Content>
			</ContextMenu.Root>
			{/each}
		</div>
	</div>
{:else}
	<table {...$tableAttrs} class="block w-full relative border-spacing-0">
		<thead id="real-thead" class="sticky top-0 z-10 bg-white">
			{#each $headerRows as headerRow (headerRow.id)}
				<Subscribe rowAttrs={headerRow.attrs()} let:rowAttrs>
					<tr {...rowAttrs}>
						{#each headerRow.cells as cell (cell.id)}
							<Subscribe attrs={cell.attrs()} let:attrs props={cell.props()} let:props>
								<th
									{...attrs}
									class={`${cell.id}-col px-4 text-left align-middle font-medium text-muted-foreground ${$compactViewMode ? 'compact-view' : ''}`}
									role="button"
									tabindex="0"
									use:props.resize
									on:click={props.sort.toggle}
									class:sorted={props.sort.order !== undefined}
								>
									{#if cell.id === 'file_type'}
										<div class="header-grid justify-items-stretch items-center px-2">
											<div class="flex justify-items-start">
												<FileTypeIcon filetype="other" />
											</div>
											<div class="flex justify-end">
												{#if props.sort.order === 'asc'}
													<i class="bi bi-caret-up-fill" style="font-size: 0.5rem;" />
												{:else if props.sort.order === 'desc'}
													<i class="bi bi-caret-down-fill" style="font-size: 0.5rem;" />
												{/if}
											</div>
										</div>
									{:else}
										<ContextMenu.Root>
											<ContextMenu.Trigger>
												<div class="header-grid justify-items-stretch items-center px-2">
													<div class="flex justify-items-start">
														<Render of={cell.render()} />
													</div>
													<div class="flex justify-end">
														{#if props.sort.order === 'asc'}
															<i class="bi bi-caret-up-fill" style="font-size: 0.5rem;" />
														{:else if props.sort.order === 'desc'}
															<i class="bi bi-caret-down-fill" style="font-size: 0.5rem;" />
														{/if}
													</div>
												</div>
											</ContextMenu.Trigger>
											<ContextMenu.Content>
												{#each columnsArray as col}
													<ContextMenu.Item on:click={() => {showHideColumn(col.id)}}>
														<Check class={`mr-2 h-3 w-3 ${hideForId[col.id] ? 'text-white' : ''}`} />{col.header}
													</ContextMenu.Item>
												{/each}
											</ContextMenu.Content>
										</ContextMenu.Root>
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
		{#if $documentsShown.length > 0}
			<tbody {...$tableBodyAttrs}>
				{#each $pageRows as row (row.id)}
					<Subscribe rowAttrs={row.attrs()} let:rowAttrs>
						<ContextMenu.Root>
							<ContextMenu.Trigger>
								<tr
									{...rowAttrs}
									id={stringToHash($documentsShown[Number(row.id)].path)}
									class={`table-row result-${Number(row.id)}`}
									role="button"
									tabindex="0"
									on:focus={(e) => clickRow(e, $shiftKeyPressed)}
									on:click={(e) => clickRow(e, $shiftKeyPressed)}
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
														<span class="flex items-center gap-1">
															<i class="bi bi-check-circle fs-small" title="Item contents scanned" style="font-size: 8px; color: var(--bs-success);"></i>
															<Render of={cell.render()} />
														</span>
													{:else}
														<span><Render of={cell.render()} /></span>
													{/if}
												{:else if cell.id === 'path'}
													<button class="w-full text-left truncate hover:underline hover:cursor-pointer" on:click={() => openFileFolder(cell.render().toString())}>
														<Render of={formatPath(cell.render().toString())} />
													</button>
												{:else}
													<span><Render of={cell.render()} /></span>
												{/if}
											</td>
										</Subscribe>
									{/each}
								</tr>
							</ContextMenu.Trigger>
							<ContextMenu.Content>
								{#if $documentsShown[Number(row.id)].file_type !== 'folder' && $documentsShown[Number(row.id)].last_parsed !== 0}
									<ContextMenu.Item on:click={() => {$showResultTextPreview = true; $selectedResult = $documentsShown[Number(row.id)];}}>
										Show Preview
									</ContextMenu.Item>
								{/if}
								<ContextMenu.Item>
									Open {$documentsShown[Number(row.id)].file_type === 'folder' ? 'Folder' : 'File'}
								</ContextMenu.Item>
								<ContextMenu.Sub>
									<ContextMenu.SubTrigger>Ignore</ContextMenu.SubTrigger>
									<ContextMenu.SubContent class="w-48">
										<ContextMenu.Item>Ignore this {row.cells[0].render().toString() === 'folder' ? 'folder' : 'file'}</ContextMenu.Item>
										<ContextMenu.Item>Ignore parent folder</ContextMenu.Item>
										{#if row.cells[0].render().toString() !== 'folder'}
											<ContextMenu.Item>Ignore this file's text</ContextMenu.Item>
										{/if}
									</ContextMenu.SubContent>
								</ContextMenu.Sub>
							</ContextMenu.Content>
						</ContextMenu.Root>
					</Subscribe>
				{/each}
			</tbody>
		{/if}
	</table>
{/if}

<div class="absolute w-full bottom-0 bg-white z-100 flex items-center justify-center space-x-4 p-2">
	<Button
		variant="outline"
		size="sm"
		class="text-sm"
		id="previous-page-results"
		on:click={() => ($pageIndex = $pageIndex - 1)}
		disabled={!$hasPreviousPage}>Previous</Button
	>
	<Label class="font-normal text-sm">Page {$pageIndex + 1}</Label>
	<Button
		variant="outline"
		size="sm"
		class="text-sm"
		id="next-page-results"
		disabled={!$hasNextPage && $noMoreResults}
		on:click={async () => {
			let currentPage = $pageIndex;
			if (!$hasNextPage && !$noMoreResults) { await loadMoreResults(); }
			if ($hasNextPage) { $pageIndex = currentPage + 1; }
		}}>
			{#if $searchInProgress}
				<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
			{:else}
				Next
			{/if}
		</Button
	>
</div>

{#key $selectedResult}
	<ResultTextPreview open={$showResultTextPreview} />
{/key}

<style lang="scss">
	tr {
		cursor: default;
		outline: none;
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
		width: 1px;
		position: absolute;
		top: 0%;
		bottom: 0;
		right: -4px;
		height: 100%;
		background: black;
		opacity: 0.05;
	}

	.header-grid {
		display: grid; 
		grid-template-columns: 1.5fr 0.5fr; 
		grid-template-rows: 1fr; 
		gap: 0px 0px; 
		grid-template-areas: 
			". ."; 
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
		// max-height: calc(100vh - 170px);
	}


	:global::-webkit-scrollbar {
		width: 0px;
		background: transparent; /* make scrollbar transparent */
	}

	#parent-grid {
    overflow-x: hidden; /* Hide horizontal scrollbar */
    overflow-y: auto; /* Enable vertical scrolling */
		max-height: 60svh;
  }
  .img-thumbnail {
    max-height: 72px;
    max-width: 96px;
  }
  .img-thumbnail.compact-view {
    max-height: 48px;
    max-width: 64px;
  }
  .file-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(100px, 1fr));
  }
  .filename {
    font-size: 0.75rem;
    width: 100px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
