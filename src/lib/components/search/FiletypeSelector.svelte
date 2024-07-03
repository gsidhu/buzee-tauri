<script lang="ts">
  import * as Select from "$lib/components/ui/select";
	import { Label } from "$lib/components/ui/label/index.js";
  import { onMount } from 'svelte';
	import {
		documentsShown,
		searchQuery,
		filetypeShown,
		resultsPerPage,
		allowedExtensions,
		dateLimitUNIX,
	} from '$lib/stores';
	import { getDocumentsFromDB, triggerSearch } from '$lib/utils/dbUtils';
	import { trackEvent } from '@aptabase/web';
	import FileCategoryIcon from "../ui/FileCategoryIcon.svelte";

	let selectedFiletypeOption = { value: $filetypeShown, label: ($filetypeShown === 'any' ? "All" : $filetypeShown.slice(0,1).toUpperCase() + $filetypeShown.slice(1)) };

	async function showDocsForFiletype(value: {}) {
    console.log(value);
    console.log($dateLimitUNIX);
    $filetypeShown = value.toString();
		trackEvent('click:showDocsForFileType');
		if ($searchQuery === '' && $dateLimitUNIX.start === '' && $dateLimitUNIX.end === '') {
			$documentsShown = await getDocumentsFromDB(0, $resultsPerPage);
		} else {
			triggerSearch();
		}
	}

	onMount(async () => {
		if ($filetypeShown === undefined) $filetypeShown = "any";
	});
</script>

<div class="flex flex-col w-full">
	<Label class="mb-2">Filetype</Label>
	<Select.Root bind:selected={selectedFiletypeOption} onSelectedChange={(v) => v?.value ? showDocsForFiletype(v.value) : showDocsForFiletype("any")}>
		<Select.Trigger class="">
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