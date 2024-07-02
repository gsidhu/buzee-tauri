<script lang="ts">
	import { compactViewMode, statusMessage, allowedExtensions } from '$lib/stores';
	// @ts-ignore
	import { createTable, Subscribe, Render } from 'svelte-headless-table';
	import { readable } from 'svelte/store';
	import { invoke } from '@tauri-apps/api/core';

	const table = createTable(readable($allowedExtensions.items));
	const columns = table.createColumns([
		table.column({
			header: 'File Type',
			accessor: 'file_type'
		}),
		table.column({
			header: 'Category',
			accessor: 'file_type_category'
		}),
		table.column({
			header: 'Enabled',
			accessor: 'file_type_allowed',
			cell: ({ value }: { value: boolean }) => value ? 'Yes' : 'No'
		}),
    // table.column({
		// 	header: 'Added By User',
		// 	accessor: 'added_by_user',
		// 	cell: ({ value }: { value: boolean }) => value ? 'Yes' : 'No'
		// })
	]);
	const { flatColumns, headerRows, rows, tableAttrs, tableBodyAttrs, pluginStates } = table.createViewModel(columns);

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
		{#if $rows.length > 0 }
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
                  <span><Render of={cell.render()} /></span>
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
		</tbody>
		{/if}
	</table>

<style lang="scss">
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