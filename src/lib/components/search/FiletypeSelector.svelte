<script lang="ts">
  import * as Select from "$lib/components/ui/select";
	import { Label } from "$lib/components/ui/label/index.js";
  import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import {
		documentsShown,
		searchQuery,
		filetypeShown,
		resultsPageShown,
		resultsPerPage,
		searchInProgress,
		allowedExtensions,
		base64Images
	} from '$lib/stores';
	import { getDocumentsFromDB, searchDocuments } from '$lib/utils/dbUtils';
	import { categoriseExtensions, setExtensionCategory } from '$lib/utils/miscUtils';
	import { trackEvent } from '@aptabase/web';
	import FileCategoryIcon from "../ui/FileCategoryIcon.svelte";

	async function showDocsForFiletype(value: {}) {
    console.log(value);
    $filetypeShown = value.toString();
		trackEvent('click:showDocsForFileType');
		$searchInProgress = true;
		$base64Images = {};
		let filetypeToGet = $filetypeShown;
		if (filetypeToGet !== 'any') {
			filetypeToGet = setExtensionCategory($filetypeShown, $allowedExtensions);
		}
		if ($searchQuery === '') {
			$documentsShown = await getDocumentsFromDB(0, $resultsPerPage, filetypeToGet);
		} else {
			$documentsShown = await searchDocuments(
				$searchQuery,
				$resultsPageShown,
				$resultsPerPage,
				filetypeToGet
			);
		}
		$searchInProgress = false;
	}

	onMount(async () => {
		$filetypeShown = "any";
		// Get list of available extensions from main process
		invoke('get_allowed_filetypes').then((res) => {
			// @ts-ignore
			$allowedExtensions = categoriseExtensions(JSON.parse(res));
			console.log("ext:", $allowedExtensions);
		});
	});
</script>

<div class="flex flex-col w-full">
	<Label class="mb-2">Filetype</Label>
	<Select.Root onSelectedChange={(v) => v?.value ? showDocsForFiletype(v.value) : showDocsForFiletype("any")}>
		<Select.Trigger class="" bind:value={$filetypeShown}>
			<Select.Value placeholder="All"/>
		</Select.Trigger>
		<Select.Content>
			<Select.Item value="any">
				<FileCategoryIcon category="any" className="mr-2 h-4 w-4" />
				All
			</Select.Item>
			{#each $allowedExtensions.categories as category}
				<Select.Item value="{category}">
					<FileCategoryIcon category={category} className="mr-2 h-4 w-4" />
					{#if category === "audio"}
						Audio
					{:else}
						{category.slice(0,1).toUpperCase() + category.slice(1)}s
					{/if}
				</Select.Item>
			{/each}
		</Select.Content>
	</Select.Root>
</div>