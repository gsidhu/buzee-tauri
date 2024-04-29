<script lang="ts">
	import { compactViewMode, statusMessage, ignoredPaths } from '$lib/stores';
	// @ts-ignore
	import { createTable, Subscribe, Render } from 'svelte-headless-table';
	import { readable } from 'svelte/store';
	import { invoke } from '@tauri-apps/api/core';

	const table = createTable(readable($ignoredPaths));
	const columns = table.createColumns([
		table.column({
			header: 'Path',
			accessor: 'path'
		}),
		table.column({
			header: 'Ignore Scanning',
			accessor: 'ignore_indexing',
			cell: ({ value }: { value: boolean }) => value ? 'Yes' : 'No'
		}),
		table.column({
			header: 'Ignore Content',
			accessor: 'ignore_content',
			cell: ({ value }: { value: boolean }) => value ? 'Yes' : 'No'
		})
	]);
	const { flatColumns, headerRows, rows, tableAttrs, tableBodyAttrs, pluginStates } = table.createViewModel(columns);

	let isSelected: StringBooleanObject = {};
	for (let i=0; i < $ignoredPaths.length; i++) {
		isSelected[i] = false;
	}

	async function removeFromList() {
		let pathsToRemove: String[] = [];
		for (let i=0; i < $ignoredPaths.length; i++) {
			if (isSelected[i]) {
				pathsToRemove.push($ignoredPaths[i].path);
			}
		}
		if (pathsToRemove.length > 0) {
			$statusMessage = "Removing items from list...";
			invoke("remove_from_ignore_list", { paths: pathsToRemove }).then((res) => {
				console.log(res);
				$statusMessage = "Removed!";
				// filter all pathsToRemove from $ignoredPaths
				$ignoredPaths = $ignoredPaths.filter((path) => !pathsToRemove.includes(path.path));
				setTimeout(() => {
					$statusMessage = "";
				}, 2000);
			});
		}
	}

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
									tabindex="0"
								>
									<Render of={cell.render()} />
								</th>
							</Subscribe>
						{/each}
					</tr>
				</Subscribe>
			{/each}
		</thead>
		{#if $ignoredPaths.length > 0 }
		<tbody {...$tableBodyAttrs}>
			{#each $rows as row (row.id)}
				<Subscribe rowAttrs={row.attrs()} let:rowAttrs rowProps={row.props()} let:rowProps>
					<tr
						{...rowAttrs}
						class="table-row"
						role="button"
						tabindex="0"
					>
						{#each row.cells as cell (cell.id)}
							<Subscribe attrs={cell.attrs()} let:attrs>
								<td {...attrs} 
									class={`${cell.id}-col ${$compactViewMode ? 'compact-view' : ''} ${cell.id === 'path' ? '' : 'text-center' }`}
									title={String(cell.render())}
								>
									{#if cell.id === 'path' }
										<span class="d-flex align-items-center gap-2">
											<input id={`select-row-${row.id}`} type="checkbox" bind:checked={isSelected[row.id]} />
											<label class="form-check-label" for={`select-row-${row.id}`}>
												<Render of={cell.render()} />
											</label>
											<!-- <span><Render of={cell.render()} /></span> -->
										</span>
									{:else}
										<span><Render of={cell.render()} /></span>
									{/if}
								</td>
							</Subscribe>
						{/each}
					</tr>
				</Subscribe>
			{/each}
		</tbody>
		{:else}
		<tbody>
			<tr>
				<td colspan="3" class="text-center">No items added yet!</td>
			</tr>
			<tr>
				<td colspan="3" class="text-center">You can add items to the Ignore List by right-clicking on a search result</td>
			</tr>
		</tbody>
		{/if}
	</table>
	{#if $ignoredPaths.length > 0 }
		<div class="text-center">
			<!-- <button type="button" class="btn-sm" on:click={() => removeFromList()}>Remove selected items from list</button> -->
			<button class="btn btn-sm link-danger px-0" on:click={() => removeFromList()}>
				Remove selected items from list
			</button>
		</div>
	{/if}

<style lang="scss">
	.btn.link-danger:hover {
		text-decoration: underline;
	}
	.path-col {
		width: 50vw;
		max-width: 50vw;
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
			// overflow: hidden;
			// text-overflow: ellipsis;
			// white-space: nowrap;
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
</style>