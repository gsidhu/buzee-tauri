<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import SvelteTable from '$lib/components/results/svelteTable.svelte';
	import IconsGrid from './iconsGrid.svelte';
	import { documentsShown, showIconGrid, preferLastOpened, noMoreResults } from '$lib/stores';
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

{#key $documentsShown || $preferLastOpened}
	<div class="overflow-x-auto w-full max-h-full block">
		{#if $showIconGrid}
			<IconsGrid />
		{:else}
			<SvelteTable />
		{/if}
	</div>
{/key}