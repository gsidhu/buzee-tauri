<script lang="ts">
	import { onMount } from 'svelte';
	// TOOD: Get allowedExtensions from the main process
	import {
		documentsShown,
		searchQuery,
		filetypeShown,
		resultsPageShown,
		resultsPerPage,
		searchInProgress
	} from '$lib/stores';
	import { getDocumentsFromDB, searchDocuments } from '$lib/utils/dbUtils';
	import { sendEvent } from '../../../utils/firebase';

	export let searchBar = true;

	let allowedExtensions: string[] = [];

	async function showDocsForFiletype() {
		// TODO: The type argument can either be a '.any' (which becomes undefined)
		// or a single extension (e.g. '.pdf', which becomes 'pdf')
		// or a list of extensions (e.g. '.pdf,.docx', which should be double-quoted and comma separated like '"pdf","docx"')
		// because the SQL IN query expects a list of double-quoted strings
		$searchInProgress = true;
		if ($searchQuery === '') {
			$documentsShown = await getDocumentsFromDB(0, $resultsPerPage, $filetypeShown.slice(1));
		} else {
			$documentsShown = await searchDocuments(
				$searchQuery,
				$resultsPageShown,
				$resultsPerPage,
				$filetypeShown.slice(1)
			);
		}
		sendEvent('click:showDocsForFileType', {
			searchQuery: $searchQuery,
      filetypeShown: $filetypeShown.slice(1),
			resultsPageShown: $resultsPageShown,
		});
		$searchInProgress = false;
	}

	onMount(async () => {
		// Get list of available extensions from main process
		// allowedExtensions = await window.electronAPI?.getAvailableExtensions();
		allowedExtensions = ['csv', 'docx', 'key', 'md', 'numbers', 'pages', 'pdf', 'pptx', 'txt', 'xlsx', 'xls'];
		console.log('allowedExtensions:', allowedExtensions);
	});
</script>

{#if searchBar}
	<select
		class="d-flex filetype-select"
		aria-label="Filetype"
		bind:value={$filetypeShown}
		on:change={() => showDocsForFiletype()}
	>
		<option value=".any">.any</option>
		{#each allowedExtensions as extension}
			<option value=".{extension}">.{extension}</option>
		{/each}
	</select>
{:else}
	<div class="filetype-select-wrapper">
		<button type="button" aria-hidden="false" tabindex="-1" class="filetype-select-label"
			><i class="bi bi-filter" /></button
		>
		<select
			class="filetype-select-table"
			aria-label="Filetype"
			bind:value={$filetypeShown}
			on:change={() => showDocsForFiletype()}
		>
			<option value=".any">.any</option>
			{#each allowedExtensions as extension}
				<option value=".{extension}">.{extension}</option>
			{/each}
		</select>
	</div>
{/if}

<style lang="scss">
	.filetype-select-wrapper {
		position: relative;
		display: inline-block;
	}
	.filetype-select-table {
		position: absolute;
		left: 0;
		width: 100%;
		height: 100%;
		opacity: 0;
		cursor: pointer;
	}
	button.filetype-select-label {
		all: unset;
	}
	.filetype-select-label {
		position: absolute;
		left: 10px;
		top: 2px;
		pointer-events: none;
	}
	.filetype-select {
		flex: 0;
		border: 0;
		background-color: var(--search-bg);
	}
	.filetype-select:focus-visible {
		border: 0;
		outline: 0;
	}
	.filetype-select,
	.filetype-select:focus-visible {
		color: var(--purple);
		font-weight: 600;
	}
	.filetype-select::-ms-expand {
		display: none;
	}
	.filetype-select::-webkit-outer-spin-button,
	.filetype-select::-webkit-inner-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}
	.filetype-select {
		-moz-appearance: none;
		-webkit-appearance: none;
		appearance: none;
	}
</style>
