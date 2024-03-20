<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
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

	let allowedExtensions: FileTypesDropdown = {
		categories: [],
		items: []
	};
	
	function categoriseExtensions(received_filetypes: FileTypes[]) {
		received_filetypes.forEach((extension) => {
			if (allowedExtensions.categories.indexOf(extension.file_type_category) === -1) {
				allowedExtensions.categories.push(extension.file_type_category);
			}
		});
		allowedExtensions.items = received_filetypes;
	}

	async function showDocsForFiletype() {
		$searchInProgress = true;
		if ($searchQuery === '') {
			$documentsShown = await getDocumentsFromDB(0, $resultsPerPage, $filetypeShown);
		} else {
			$documentsShown = await searchDocuments(
				$searchQuery,
				$resultsPageShown,
				$resultsPerPage,
				$filetypeShown
			);
		}
		sendEvent('click:showDocsForFileType', {
			searchQuery: $searchQuery,
      filetypeShown: $filetypeShown,
			resultsPageShown: $resultsPageShown,
		});
		$searchInProgress = false;
	}

	onMount(async () => {
		// Get list of available extensions from main process
		invoke('get_allowed_filetypes').then((res) => {
			// @ts-ignore
			categoriseExtensions(JSON.parse(res));
			console.log('allowedExtensions:', allowedExtensions);
		});
	});
</script>

{#if searchBar}
	<select
		class="d-flex filetype-select"
		aria-label="Filetype"
		bind:value={$filetypeShown}
		on:change={() => showDocsForFiletype()}
	>
		<option value="any" selected>any</option>
		{#each allowedExtensions.categories as category}
			<option value="{category}">{category}</option>
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
			<option value="any">any</option>
			{#each allowedExtensions.categories as category}
				<option value="{category}">{category}</option>
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
