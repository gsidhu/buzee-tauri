<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import SvelteTable from '$lib/components/results/svelteTable.svelte';
	import IconsGrid from './iconsGrid.svelte';
	import { documentsShown, showIconGrid, preferLastOpened, noMoreResults } from '$lib/stores';
	import LoadMoreButton from './LoadMoreButton.svelte';
	import { goto } from '$app/navigation';

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
	<div class="overflow-x-auto w-full h-full max-h-full block">
		{#if $documentsShown.length > 0}
			{#if $showIconGrid}
				<IconsGrid />
			{:else}
				<SvelteTable />
			{/if}
		{:else}
			<div class="flex flex-col h-full px-4 py-2 mx-auto items-center justify-center max-h-[75vh]">
				<img id="buzee-logo-img" class="w-25 my-2" src="/Buzee Logo.png" alt="No Results" />
				<h3 class="text-lg">No Results</h3>
				<div class="flex flex-col text-light-emphasis text-center small gap-2">
					<span>Try modifying your query? You can be more specific like –</span>
					<span><code>last year "annual report" -pdf</code></span>
				</div>
				<button type="button" class="my-2 btn py-1 px-2 leading-tight text-xs purple border-hover-purple border-2 border-gray-100 rounded" on:click={() => goto('/magic/tips')}>View all tips and shortcuts</button>
			</div>
		{/if}
	</div>
{/key}

<style>
	#buzee-logo-img {
		max-width: 200px;
	}
</style>