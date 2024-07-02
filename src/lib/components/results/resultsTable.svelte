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
	<div class="flex flex-col items-center overflow-y-auto">
		<div class={`results-container flex-[0_0_90%]`}>
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
		max-height: 70svh;
	}

	.modal-dialog-centered {
		max-width: 80vw;
	}
</style>
