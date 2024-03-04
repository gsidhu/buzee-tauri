<script lang="ts">
	import { onMount } from 'svelte';
	import {
		searchQuery,
		filetypeShown,
		documentsShown,
		resultsPageShown,
		resultsPerPage,
		searchInProgress,
		compactViewMode
	} from '$lib/stores';
	import { searchDocuments } from '$lib/utils/dbUtils';
	let isInputFocused = false;
	let searchInputRef: HTMLInputElement; // a reference to the input element that allows updating the DOM without running a querySelector.

	import FiletypeDropdown from './FiletypeDropdown.svelte';
	import { sendEvent } from '../../../utils/firebase';

	async function triggerSearch() {
		$searchInProgress = true;
		sendEvent('search-triggered', {
			searchQuery: $searchQuery,
			filetypeShown: $filetypeShown.slice(1),
			resultsPageShown: $resultsPageShown
		});
		const results = await searchDocuments(
			$searchQuery,
			$resultsPageShown,
			$resultsPerPage,
			$filetypeShown.slice(1)
		);
		sendEvent('search-results', { searchQuery: $searchQuery, resultsLength: results?.length });
		$documentsShown = results;
		$searchInProgress = false;
		// $resultsPageShown += 1;
		// if ($resultsPageShown === 1) {
		//   $documentsShown = results;
		// } else {
		//   $documentsShown = [...$documentsShown, ...results];
		// }
		// searchInputRef.blur();
	}

	function clearSearchQuery() {
		$searchQuery = '';
		sendEvent('click:clearSearchQuery');
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
	});
</script>

<div id="search-bar-wrapper" class={`rounded-3 no-drag ${$compactViewMode ? 'compact-view' : ''}`}>
	<i class="bi bi-search px-1" aria-label="Search" aria-hidden="true" />
	<!-- actual search box -->
	<div id="actual-search-box" class="d-flex flex-grow-1">
		<form class="d-flex flex-fill" role="search" on:submit|preventDefault={() => triggerSearch()}>
			<input
				id="search-input"
				type="search"
				class="d-flex"
				placeholder="Search Documents"
				aria-label="Search Documents"
				bind:this={searchInputRef}
				bind:value={$searchQuery}
				on:focus={() => {
					isInputFocused = true;
				}}
				on:blur={() => {
					isInputFocused = false;
				}}
			/>
			<FiletypeDropdown searchBar={true} />
		</form>
		<div class="d-flex align-items-center justify-content-center">
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
		border: 0;
		width: 100%;
		font-size: 1rem;
		background-color: var(--search-bg);
		background-clip: padding-box;
		appearance: none;
		border: 2px solid #dee2e6;
		transition: border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;

		&:focus-within {
			border: 2px solid var(--light-purple);
			color: var(--purple);
		}
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
