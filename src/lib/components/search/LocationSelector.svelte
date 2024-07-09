<script lang="ts">
  import * as Select from "$lib/components/ui/select";
	import { Label } from "$lib/components/ui/label/index.js";
  import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import {
		locationShown,
		allowedLocations,
	} from '$lib/stores';
	import { getDocumentsFromDB, searchDocuments } from '$lib/utils/dbUtils';
	import { categoriseExtensions, setExtensionCategory } from '$lib/utils/miscUtils';
	import { trackEvent } from '@aptabase/web';
	import FileCategoryIcon from "../ui/FileCategoryIcon.svelte";

	let selectedLocationOption = { value: $locationShown, label: ($locationShown === 'any' ? "All" : $locationShown.slice(0,1).toUpperCase() + $locationShown.slice(1)) };

	async function showDocsForLocation(value: {}) {
    console.log(value);
    $locationShown = value.toString();
		selectedLocationOption = { value: $locationShown, label: $locationShown };
		return;
	}

	onMount(async () => {
	});
</script>

<div class="flex flex-col w-full">
	<Label class="mb-2 font-medium">Location</Label>
	<Select.Root bind:selected={selectedLocationOption} onSelectedChange={(v) => v?.value ? showDocsForLocation(v.value) : showDocsForLocation("any")}>
		<Select.Trigger class="">
			<Select.Value placeholder="All" />
		</Select.Trigger>
		<Select.Content>
			<Select.Item value={"any"}>
				<FileCategoryIcon category="any" className="mr-2 h-4 w-4" />
				All
			</Select.Item>
			{#each $allowedLocations as category}
				<Select.Item value={category}>
					<FileCategoryIcon category={category} className="mr-2 h-4 w-4" />
          {category.slice(0,1).toUpperCase() + category.slice(1)}
				</Select.Item>
			{/each}
		</Select.Content>
	</Select.Root>
</div>