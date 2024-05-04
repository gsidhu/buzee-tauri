<script lang="ts">
	import { onMount } from 'svelte';
	import {
		searchQuery,
		filetypeShown,
		documentsShown,
		resultsPageShown,
		resultsPerPage,
		searchInProgress,
		compactViewMode,
		allowedExtensions,
		searchSuggestions,
		base64Images,
		userPreferences
	} from '$lib/stores';
	import { searchDocuments } from '$lib/utils/dbUtils';
	import { setExtensionCategory } from '$lib/utils/miscUtils';
	import FiletypeDropdown from './FiletypeDropdown.svelte';
	import SearchSuggestions from './searchSuggestions.svelte';
	import { trackEvent } from '@aptabase/web';
	import { invoke } from '@tauri-apps/api/core';

	let isInputFocused = false;
	let searchInputRef: HTMLInputElement; // a reference to the input element that allows updating the DOM without running a querySelector

	// Limiting searchSuggestions to five items so don't have to implement a scroll
  let selectedSuggestionItem = -1;
	let getSuggestions = true;

	$: if ($searchQuery.length >= 3 && getSuggestions && $userPreferences.show_search_suggestions) {
		getSearchSuggestions();
	} else if ($searchQuery.length < 3) {
		$searchSuggestions = [];
	}

	async function getSearchSuggestions() {
		let removeSpecialChars = $searchQuery.replace(/[^a-zA-Z0-9 ]/g, '');
		$searchSuggestions = await invoke('get_search_suggestions', { query: removeSpecialChars });
		$searchSuggestions = [...new Set($searchSuggestions)]; // remove duplicates
	}

	async function triggerSearch() {
		$resultsPageShown = 0; // reset the page number on each new search
		$searchInProgress = true;
		$base64Images = {};
		trackEvent('search-triggered', {
			filetypeShown: $filetypeShown,
			resultsPageShown: $resultsPageShown
		});
		let filetypeToGet = $filetypeShown;
		if (filetypeToGet !== 'any') {
			filetypeToGet = setExtensionCategory($filetypeShown, $allowedExtensions);
		}
		let results = await searchDocuments(
			$searchQuery,
			$resultsPageShown,
			$resultsPerPage,
			filetypeToGet
		);
		$documentsShown = results;
		$searchInProgress = false;
		searchInputRef.blur();
	}

	function clearSearchQuery() {
		$searchQuery = '';
		$searchSuggestions = [];
	}

	onMount(() => {
		// get the query from the url
		const urlParams = new URLSearchParams(window.location.search);
		const query = urlParams.get('q');
		const highlightSearchBar = urlParams.get('highlight-search-bar');
		if (query) {
			$searchQuery = query;
			triggerSearch();
		}
		if (highlightSearchBar) {
			searchInputRef.focus();
		}
		// add event listener to #search-input to handle arrowdown and arrowup
		searchInputRef.addEventListener('keydown', (e) => {
			if (e.key === 'ArrowDown') {
				if (selectedSuggestionItem < $searchSuggestions.length - 1) {
					selectedSuggestionItem += 1;
					getSuggestions = false;
					$searchQuery = $searchSuggestions[selectedSuggestionItem];
				}
			} else if (e.key === 'ArrowUp') {
				if (selectedSuggestionItem > 0) {
					selectedSuggestionItem -= 1;
					getSuggestions = false;
					$searchQuery = $searchSuggestions[selectedSuggestionItem];
					setTimeout(function() {
						// set the cursor to the end of the input because browsers set it to the beginning when you press ArrowUp
						searchInputRef.setSelectionRange($searchQuery.length, $searchQuery.length);
					}, 1);
				}
			} else {
				selectedSuggestionItem = -1; // on any other key press, reset the suggestion selection
				getSuggestions = true;
			}
		});
	});
</script>

<div id="search-bar-outer-wrapper">
	<div id="search-bar-wrapper" class={`rounded-3 no-drag ${$compactViewMode ? 'compact-view' : ''}`}>
		<i class="bi bi-search px-1" aria-label="Search" aria-hidden="true" />
		<!-- actual search box -->
		<div id="actual-search-box" class="d-flex flex-grow-1">
			<form class="d-flex flex-fill" role="search" on:submit|preventDefault={() => triggerSearch()}>
				<input
					id="search-input"
					type="search"
					class="d-flex"
					placeholder="Search"
					aria-label="Search"
					spellcheck="false"
					bind:this={searchInputRef}
					bind:value={$searchQuery}
					on:focus={() => {
						isInputFocused = true;
					}}
					on:blur={() => {
						// blur after timeout so that clicks on suggested items get registered
						setTimeout(() => {
							isInputFocused = false;
						}, 200);
					}}
				/>
				<FiletypeDropdown searchBar={true} />
			</form>
			<div class="d-flex align-items-center justify-content-center clear-search-div">
				{#if $searchQuery !== ''}
					<button class="btn clear-search" on:click={() => clearSearchQuery()}>
						<i class="bi bi-x-circle-fill px-1" aria-label="Clear Search" />
					</button>
				{:else}
					<button class="btn" tabindex="-1" aria-hidden="true" disabled>
						<i
							id="placeholder-clear-btn"
							class="bi bi-x-circle-fill px-1"
							aria-label="Invisible Placeholder"
							aria-hidden="true"
						/>
					</button>
				{/if}
			</div>
		</div>
	</div>
	{#if $userPreferences.show_search_suggestions}
		<SearchSuggestions
			isSearchSuggestionsVisible={isInputFocused && $searchSuggestions.length > 0}
			{selectedSuggestionItem}
			{triggerSearch}
		/>
	{/if}
</div>

<style lang="scss">
	.btn:disabled {
		border: none !important;
	}
	input[type='search'] {
		border: 0;
		background-color: var(--search-bg);
	}
	input[type='search']:focus-visible {
		border: 0;
		outline: 0;
	}
	/* clears the ‘X’ from Internet Explorer */
	input[type='search']::-ms-clear {
		display: none;
		width: 0;
		height: 0;
	}
	input[type='search']::-ms-reveal {
		display: none;
		width: 0;
		height: 0;
	} /* clears the ‘X’ from Chrome */
	input[type='search']::-webkit-search-decoration,
	input[type='search']::-webkit-search-cancel-button,
	input[type='search']::-webkit-search-results-button,
	input[type='search']::-webkit-search-results-decoration {
		display: none;
	}

	#search-input {
		flex: 11;
	}

	#search-bar-wrapper.compact-view {
		font-size: 0.9rem;
	}

	#search-bar-wrapper {
		display: flex;
		width: 100%;
		font-size: 1rem;
		background-color: var(--search-bg);
		background-clip: padding-box;
		appearance: none;
		border: 2px solid #dee2e6;
		border-radius: 8px;
		transition: border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;

		&:focus-within {
			border-color: var(--light-purple);
			color: var(--purple);
		}
	}

	#search-bar-outer-wrapper {
		width: 100%;
		position: relative;
	}

	.btn {
		padding: 0;
		margin: 0;
		font-size: small;
		cursor: default;
		animation: fade-in-animation 0.25s ease-in-out;
	}

	.clear-search:focus {
		color: var(--hot-pink);
	}

	.clear-search-div {
		width: 24px;
	}

	#placeholder-clear-btn {
		color: transparent;
	}

	@keyframes fade-in-animation {
		0% {
			opacity: 0;
			scale: 0.8;
		}

		100% {
			opacity: 1;
			scale: 1;
		}
	}
</style>
