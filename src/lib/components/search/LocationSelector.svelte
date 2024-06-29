<script lang="ts">
  import * as Select from "$lib/components/ui/select";
	import { Label } from "$lib/components/ui/label/index.js";
  import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import {
		documentsShown,
		searchQuery,
		locationShown,
		resultsPageShown,
		resultsPerPage,
		searchInProgress,
		allowedLocations,
		base64Images
	} from '$lib/stores';
	import { getDocumentsFromDB, searchDocuments } from '$lib/utils/dbUtils';
	import { categoriseExtensions, setExtensionCategory } from '$lib/utils/miscUtils';
	import { trackEvent } from '@aptabase/web';
	import FileCategoryIcon from "../ui/FileCategoryIcon.svelte";

	async function showDocsForLocation(value: {}) {
    return;
    // console.log(value);
    // $locationShown = value.toString();
		// trackEvent('click:showDocsForLocation');
		// $searchInProgress = true;
		// $base64Images = {};
		// let filetypeToGet = $locationShown;
		// if (filetypeToGet !== 'any') {
		// 	filetypeToGet = setExtensionCategory($locationShown, $allowedLocations);
		// }
		// if ($searchQuery === '') {
		// 	$documentsShown = await getDocumentsFromDB(0, $resultsPerPage, filetypeToGet);
		// } else {
		// 	$documentsShown = await searchDocuments(
		// 		$searchQuery,
		// 		$resultsPageShown,
		// 		$resultsPerPage,
		// 		filetypeToGet
		// 	);
		// }
		// $searchInProgress = false;
	}

	onMount(async () => {
		// Get list of available extensions from main process
		// invoke('get_allowed_filetypes').then((res) => {
		// 	// @ts-ignore
		// 	$allowedLocations = categoriseExtensions(JSON.parse(res));
		// 	console.log("ext:", $allowedLocations);
		// });
	});
</script>

<div class="flex flex-col w-full">
	<Label class="mb-2 font-medium">Location</Label>
	<Select.Root onSelectedChange={(v) => v?.value ? showDocsForLocation(v.value) : showDocsForLocation("any")}>
		<Select.Trigger class="" bind:value={$locationShown}>
			<Select.Value placeholder="All"/>
		</Select.Trigger>
		<Select.Content>
			<Select.Item value="any">
				<FileCategoryIcon category="any" className="mr-2 h-4 w-4" />
				All
			</Select.Item>
			{#each $allowedLocations as category}
				<Select.Item value="{category}">
					<FileCategoryIcon category={category} className="mr-2 h-4 w-4" />
          {category.slice(0,1).toUpperCase() + category.slice(1)}
				</Select.Item>
			{/each}
		</Select.Content>
	</Select.Root>
</div>