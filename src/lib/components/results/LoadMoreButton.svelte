<script lang="ts">
	import { Button } from "../ui/button";
  import { searchInProgress } from "$lib/stores";
  import { loadMoreResults } from '$lib/utils/dbUtils';
	import LoaderCircle from "lucide-svelte/icons/loader-circle";
</script>

<div id="load-more-btn" class="py-4 flex justify-center items-center flex-[0_0_10%]" draggable="false">
  <Button
    variant="secondary"
    class="py-1 px-2 leading-tight text-xs min-w-[120px]"
    on:click={loadMoreResults}
    disabled={$searchInProgress}
	>
		{#if $searchInProgress}
			<LoaderCircle class="mr-2 h-4 w-4 animate-spin" />
		{:else}
    	Load More Results
		{/if}
  </Button>
</div>