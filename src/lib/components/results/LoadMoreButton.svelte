<script lang="ts">
	import { Button } from "../ui/button";
  import { searchInProgress, noMoreResults, resultsPageShown, filetypeShown, allowedExtensions, searchQuery, resultsPerPage, documentsShown} from "$lib/stores";
  import { trackEvent } from '@aptabase/web';
  import { searchDocuments } from '$lib/utils/dbUtils';
  import { setExtensionCategory } from '$lib/utils/miscUtils';
	import LoaderCircle from "lucide-svelte/icons/loader-circle";

	let loading = false; 
	
  async function loadMoreResults() {
		loading = true;
		// Same function as triggerSearch, but with a different page number and appending results
		console.log("Loading more results...");
		$resultsPageShown += 1; // increment the page number on each new search
		$searchInProgress = true;
		trackEvent('loadMoreResults', {
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
		if (results.length === 0) {
			$noMoreResults = true;
		} else {
			$documentsShown = [...$documentsShown, ...results];
		}
		$searchInProgress = false;
		loading = false;
	}

</script>

<div id="load-more-btn" class="py-4 flex justify-center items-center flex-[0_0_10%]" draggable="false">
  <Button
    variant="secondary"
    class="py-1 px-2 leading-tight text-xs min-w-[120px]"
    on:click={loadMoreResults}
    disabled={$searchInProgress}
	>
		{#if loading}
			<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
		{:else}
    	Load More Results
		{/if}
  </Button>
</div>