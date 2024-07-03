<script lang="ts">
	import { onMount } from 'svelte';
	import { Input } from "$lib/components/ui/input/index.js";
	import * as Command from "$lib/components/ui/command";
	import * as Dialog from "$lib/components/ui/dialog";
	import {
		searchQuery,
		searchSuggestions,
		userPreferences,
	} from '$lib/stores';
	import { triggerSearch } from '$lib/utils/dbUtils';
	import { invoke } from '@tauri-apps/api/core';
	import { Search } from "lucide-svelte";

	let isInputFocused = false;

	let getSuggestions = true;
	let search = '';

	$: search = $searchQuery;

	$: if ($searchQuery.length >= 3 && getSuggestions && $userPreferences.show_search_suggestions) {
		getSearchSuggestions();
	} else if ($searchQuery.length < 3) {
		$searchSuggestions = [];
	}

	async function getSearchSuggestions() {
		let removeSpecialChars = $searchQuery.replace(/[^a-zA-Z0-9 ]/g, '');
		$searchSuggestions = await invoke('get_search_suggestions', { query: removeSpecialChars });
		$searchSuggestions = [...new Set($searchSuggestions)]; // remove duplicates
		// $searchSuggestions = [$searchQuery, ...$searchSuggestions]; // add the query itself to the suggestions
	}

	async function triggerSearchLocal(query: string) {
		$searchQuery = query;
		(document.querySelector('button[data-dialog-close]') as HTMLElement)?.click();
		triggerSearch();
	}

	onMount(() => {
		// get the query from the url
		const urlParams = new URLSearchParams(window.location.search);
		const query = urlParams.get('q');
		const highlightSearchBar = urlParams.get('highlight-search-bar');
		if (query) {
			$searchQuery = query;
			triggerSearchLocal(query);
		}
		if (highlightSearchBar) {
		}
	});
</script>

<form class="grid">
	<div class="relative items-center">
		<Dialog.Root>
			<Dialog.Trigger id="search-input" class="w-full">
				<Search class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
				<Input
					placeholder="Search your files, browser history, bookmarks..."
					class="w-full appearance-none bg-background pl-8 shadow-none max-w-l"
					aria-label="Search"
					bind:value={$searchQuery}
					on:input={() => {
						isInputFocused = true;
					}}
					on:blur={() => {
					}}
				/>
			</Dialog.Trigger>
			<Dialog.Content>
				<Command.Root
					filter={(value, search) => {
						if (value.includes(search) || search === $searchQuery) return 1;
						return 0;
					}}
				>
					<Command.Input
						placeholder="Search your files, browser history, bookmarks..."
						class="w-full appearance-none bg-background pl-8 shadow-none max-w-l"
						aria-label="Search"
						bind:value={$searchQuery}
					/>
						<Command.List>
							<Command.Empty>No results found.</Command.Empty>
							{#if $searchQuery.length === 0}
								<Command.Item onSelect={() => {triggerSearchLocal("");}}>
									Show Recent Files
								</Command.Item>
								<Command.Item value="last week" onSelect={(value) => {triggerSearchLocal(value);}}>
									Show Files from Last Week
								</Command.Item>
								<Command.Item value="last month" onSelect={(value) => {triggerSearchLocal(value);}}>
									Show Files from Last Month
								</Command.Item>
							{/if}
							{#if $searchQuery.length > 0}
								<Command.Group heading="Search Everywhere">
									<Command.Item value={$searchQuery} onSelect={(value) => {triggerSearchLocal(value);}}>
										{$searchQuery}
									</Command.Item>
								</Command.Group>
							{/if}
							{#if $searchSuggestions.length > 0}
								<Command.Group heading="Suggestions">
									{#each $searchSuggestions as searchItem}
										<Command.Item value={searchItem}
											onSelect={(value) => {triggerSearchLocal(value);}}
										>
											{#if searchItem.length > 50}
												{searchItem.slice(0, 30) + ' ... ' + searchItem.slice(-30)}
											{:else}
												{searchItem}
											{/if}
										</Command.Item>
									{/each}
								</Command.Group>
							{/if}
						</Command.List>
				</Command.Root>
			</Dialog.Content>
		</Dialog.Root>
	</div>
</form>

<style lang="scss">

</style>
