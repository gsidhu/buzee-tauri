<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
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
		locationShown,
		isMac
	} from '$lib/stores';
	import { triggerSearch } from '$lib/utils/dbUtils';
	import { invoke } from '@tauri-apps/api/core';
	import { Search } from "lucide-svelte";
	import Button from '../ui/button/button.svelte';
	import { extractDate } from '$lib/utils/queryParsing';
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';

	let getSuggestions = true;
	let search = '';

	$: search = $searchQuery;

	$: if ($searchQuery.length >= 3 && getSuggestions && $userPreferences.show_search_suggestions) {
		getSearchSuggestions();
	} else if ($searchQuery.length < 3) {
		$searchSuggestions = [];
	}

	async function getSearchSuggestions() {
		// only get suggestions if locationShown is computer
		if ($locationShown !== "my computer") return;
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
		// if not on /search page, go to /search page
		if ($page.route.id !== '/search' ) {
			goto('/search');
		}
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

	function setResultShortcuts() {
		let count = 2;
		// when only suggestions are shown, number from 1-9
		if ($searchQuery === '') count = 1;
		const selectedResult = document.querySelector('[data-selected="true"]');
		const selectedResultText = (selectedResult?.children[1] as HTMLElement).innerText;
		// set first option shortcut as Cmd+1
		console.log(`[data-value=${$searchQuery.replaceAll('"', '>')}]`);
		const firstResult = document.querySelector(`[data-value="${$searchQuery.replaceAll('"', '>')}"]`);
		if (firstResult && firstResult.children[1]) {
			if ((firstResult.children[1] as HTMLElement).innerText === selectedResultText) {
				(firstResult as HTMLElement).setAttribute('data-result-shortcut', 'Enter');
				(firstResult.children[1] as HTMLElement).innerText = '⏎';
				count = 1;
			} else {
				(firstResult as HTMLElement).setAttribute('data-result-shortcut', '1');
				if ($isMac) (firstResult.children[1] as HTMLElement).innerText = '⌘1';
				else (firstResult.children[1] as HTMLElement).innerText = 'Ctrl+1';
			}
		}
		// set rest of the shortcuts as 2-9 with the selected one being set as Enter
		let suggestedResults = document.querySelectorAll('[data-cmdk-group][data-value="suggestions"] > div > [data-result-shortcut]');
		suggestedResults.forEach((result, index) => {
			let innerText = (result.children[1] as HTMLElement).innerText;
			if (innerText !== selectedResultText) {
				(result as HTMLElement).setAttribute('data-result-shortcut', count.toString());
				if ($isMac) (result.children[1] as HTMLElement).innerText = `⌘${count}`;
				else (result.children[1] as HTMLElement).innerText = `Ctrl+${count}`;
				count++;
			} else {
				(result as HTMLElement).setAttribute('data-result-shortcut', 'Enter');
				(result.children[1] as HTMLElement).innerText = '⏎';
			}
		});
	}

	function handleKeydown(e: KeyboardEvent) {
		if ($page.route.id !== '/search') return;

		if (e.code === 'ArrowUp' || e.code === 'ArrowDown') {
			e.preventDefault();
			setResultShortcuts();
		}
		if (e.code.slice(0,-1) === 'Digit' && $metaKeyPressed) {
			e.preventDefault();
			const digit = e.code.slice(-1);
			if (parseInt(digit) < 9) {
				(document.querySelector(`[data-result-shortcut="${digit}"]`) as HTMLElement)?.click();
			}
		}
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

		// keyboard listeners for command shortcuts
		document.addEventListener('keydown', handleKeydown);
	});

	onDestroy(() => {
		document.removeEventListener('keydown', handleKeydown);
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
				shouldFilter={false}
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
								<Command.Group heading={`Search ${$locationShown.slice(0,1).toUpperCase() + $locationShown.slice(1)}`}>
									<Command.Item value={$searchQuery.replaceAll('"', '>')} onSelect={() => {triggerSearchLocal($searchQuery);}}>
										<span>{$searchQuery}</span>
										<Command.Shortcut>⏎</Command.Shortcut>
									</Command.Item>
								</Command.Group>
							{/if}
							{#if ($searchQuery.length === 0 || $searchSuggestions.length === 0) && $locationShown === "my computer"}
								<Command.Group heading="Suggestions">
									{#each suggestionsList as suggestion, id}
										<Command.Item data-result-shortcut="{id+1}" value={suggestion.value} onSelect={() => {triggerSearchLocal(suggestion.value);}}>
											<span>{suggestion.title}</span>
											<Command.Shortcut>{$isMac ? '⌘' : 'Ctrl+'}{id+1}</Command.Shortcut>
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
												<Command.Shortcut>{$isMac ? '⌘' : 'Ctrl+'}{id + 1}</Command.Shortcut>
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