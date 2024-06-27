<script lang="ts">
  import * as Select from "$lib/components/ui/select";
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

	// export let searchBar = true;

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
		// Get list of available extensions from main process
		invoke('get_allowed_filetypes').then((res) => {
			// @ts-ignore
			$allowedExtensions = categoriseExtensions(JSON.parse(res));
			console.log("ext:", $allowedExtensions);
		});
	});
</script>
 
<Select.Root onSelectedChange={(v) => v?.value ? showDocsForFiletype(v.value) : showDocsForFiletype("any")}>
  <Select.Trigger class="w-[180px]">
    <Select.Value placeholder="All"/>
  </Select.Trigger>
  <Select.Content>
    <Select.Item value="any">All</Select.Item>
    {#each $allowedExtensions.categories as category}
      <Select.Item value="{category}">{category}</Select.Item>
		{/each}
  </Select.Content>
</Select.Root>