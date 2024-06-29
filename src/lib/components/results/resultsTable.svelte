<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import SvelteTable from '$lib/components/results/svelteTable.svelte';
	import IconsGrid from './iconsGrid.svelte';
	import { documentsShown, showIconGrid, preferLastOpened, noMoreResults } from '$lib/stores';
	import SearchFilters from '../search/SearchFilters.svelte';
	import Separator from '../ui/separator/separator.svelte';
	import LoadMoreButton from './LoadMoreButton.svelte';

	// onMount(async () => {
	// 	if (document) {
	// 		document.body.style.overflow = 'hidden';
	// 	}
	// });

	// onDestroy(async () => {
	// 	if (document) {
	// 		document.body.style.overflow = 'auto';
	// 	}
	// });
</script>

<SearchFilters />
<Separator class="my-2 "/>
{#key $documentsShown || $preferLastOpened}
	<div class="flex flex-col items-center">
		<div class={`results-container mb-4 overflow-hidden`}>
			{#if $showIconGrid}
				<IconsGrid />
			{:else}
				<SvelteTable />
			{/if}
		</div>
		{#if $documentsShown.length > 0 || $noMoreResults === true}
			<LoadMoreButton />
		{/if}
	</div>
{/key}

<style>
	.results-container {
		overflow-x: auto;
		width: 100%;
		max-height: 60svh;
	}

	.modal-dialog-centered {
		max-width: 80vw;
	}
</style>
