<script lang="ts">

	export let isSearchSuggestionsVisible = false;
	export let selectedSuggestionItem = 0;
  export let triggerSearch = () => {};
  import {searchQuery, searchSuggestions} from '$lib/stores';

  function searchTrigger(searchItem: string) {
    // if user clicks on a suggestion, double quote the query
    $searchQuery = '"' + searchItem + '"';
    triggerSearch();
  }
</script>

<div id="search-suggestions" class:d-none={!isSearchSuggestionsVisible}>
	<ul class="d-flex list-group">
		{#each $searchSuggestions as searchItem, index}
			<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<li
				class={`btn list-group-item border-0 d-flex py-0 ${
					index === selectedSuggestionItem
						? 'selected-history-item'
						: 'unselected-history-item'
				}`}
				id={'search-suggestions-item-' + index}
        on:click={() => (searchTrigger(searchItem))}
			>
					{searchItem}
			</li>
		{/each}
  </ul>
</div>

<style lang="scss">
  #search-suggestions {
    position: absolute;
    z-index: 1050;
    width: 100%;
    top: 24px; // hack: overlap the search bar a bit to cover the bottom border
  }
	.selected-history-item {
		background-color: #e7f1ff;
	}
	.unselected-history-item {
		background-color: white;
	}
  .list-group-item {
    border-radius: 0px;
    cursor: default;
  }
  .list-group {
    overflow: auto;
    // max-height: 120px; // hack: 5 items
    border-radius: 8px;
		transition: border-color 0.15s ease-in-out, box-shadow 0.15s ease-in-out;
    border: 2px solid var(--light-purple);
    border-top: none;
    border-top-left-radius: 0% !important;
    border-top-right-radius: 0% !important;
    border-bottom-left-radius: 8px !important;
    border-bottom-right-radius: 8px !important;
  }
</style>
