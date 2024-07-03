<script lang="ts">
	import { onMount } from 'svelte';
	import { Input } from "$lib/components/ui/input/index.js";
	import * as Command from "$lib/components/ui/command";
	import * as Popover from "$lib/components/ui/popover";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import * as Dialog from "$lib/components/ui/dialog";
	import {
		dateLimitUNIX,
		searchQuery,
		searchSuggestions,
		userPreferences,
		metaKeyPressed,
		searchSuggestionsDialogOpen,
	} from '$lib/stores';
	import { triggerSearch } from '$lib/utils/dbUtils';
	import { invoke } from '@tauri-apps/api/core';
	import { Search } from "lucide-svelte";
	import Button from '../ui/button/button.svelte';
	import { extractDate } from '$lib/utils/queryParsing';

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
		$searchSuggestionsDialogOpen = false;
		let localDateLimitUNIX = extractDate(query);
		if (localDateLimitUNIX) {
			$dateLimitUNIX = localDateLimitUNIX;
		}
		triggerSearch();
	}

	const suggestionsList = [
		{
			title: 'Show Recent Files',
			value: '',
		},
		{
			title: 'Show Files from Last Week',
			value: 'last week',
		},
		{
			title: 'Show Files from Last Month',
			value: 'last month',
		},
		{
			title: 'Show Files from This Month',
			value: 'this month',
		},
		{
			title: 'Show Files from This Year',
			value: 'this year',
		},
	]

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

		// keyboard listeners for command shortcuts
		document.addEventListener('keydown', (e) => {
			if (e.code.slice(0,-1) === 'Digit' && $metaKeyPressed) {
				e.preventDefault();
				const digit = e.code.slice(-1);
				if (parseInt(digit) < 9) {
					(document.querySelector(`[data-result-shortcut="${digit}"]`) as HTMLElement)?.click();
				}
			}
		});
	});
</script>

<form class="grid">
	<div class="relative items-center">
		<Dialog.Root bind:open={$searchSuggestionsDialogOpen}>
			<Dialog.Trigger id="search-input" class="w-full">
				<Search class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
				<Input
					placeholder="Search your files, browser history, bookmarks..."
					class="w-full appearance-none bg-background pl-8 shadow-none max-w-l"
					aria-label="Search"
					bind:value={$searchQuery}
					on:blur={() => {
					}}
				/>
			</Dialog.Trigger>
			<Dialog.Content class="h-[50vh] max-h-[50vh] flex flex-col justify-between">
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
							{#if $searchQuery.length > 0}
								<Command.Group heading="Search Everywhere">
									<Command.Item value={$searchQuery.replaceAll('"', '>')} onSelect={() => {triggerSearchLocal($searchQuery);}}>
										<span>{$searchQuery}</span>
										<Command.Shortcut>⏎</Command.Shortcut>
									</Command.Item>
								</Command.Group>
							{/if}
							{#if $searchQuery.length === 0 || $searchSuggestions.length === 0}
								<Command.Group heading="Suggestions">
									{#each suggestionsList as suggestion, id}
										<Command.Item data-result-shortcut="{id+1}" value={suggestion.value} onSelect={() => {triggerSearchLocal(suggestion.value);}}>
											<span>{suggestion.title}</span>
											<Command.Shortcut>⌘{id+1}</Command.Shortcut>
										</Command.Item>
									{/each}
								</Command.Group>
							{/if}
							{#if $searchSuggestions.length > 0}
								<Command.Group heading="Suggestions">
									{#each $searchSuggestions as searchItem, id}
										<Command.Item data-result-shortcut={(id+1).toString()} value={searchItem}
											onSelect={(value) => {triggerSearchLocal(value);}}
										>
											{#if searchItem.length > 50}
												<span>{searchItem.slice(0, 30) + ' ... ' + searchItem.slice(-30)}</span>
											{:else}
												<span>{searchItem}</span>
											{/if}
											{#if id < 9}
												<Command.Shortcut>⌘{id + 1}</Command.Shortcut>
											{/if}
										</Command.Item>
									{/each}
								</Command.Group>
							{/if}
						</Command.List>
				</Command.Root>
				<div class="flex justify-center items-center">
					<img id="buzee-logo-img" class="h-6 w-6" src="/Buzee Logo.png" alt="Buzee Logo" />
				</div>
			</Dialog.Content>
		</Dialog.Root>
	</div>
</form>