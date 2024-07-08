<script lang="ts">
	import { fade } from 'svelte/transition';
	import ResultsTable from '$lib/components/results/resultsTable.svelte';
	import SearchFilters from '$lib/components/search/SearchFilters.svelte';
	import Separator from '$lib/components/ui/separator/separator.svelte';
	import LoadMoreButton from '$lib/components/results/LoadMoreButton.svelte';
	import { documentsShown, noMoreResults, searchQuery, dateLimitUNIX, filetypeShown } from '$lib/stores';
	import { onMount } from 'svelte';
	import { triggerSearch } from '$lib/utils/dbUtils';

	onMount(() => {
		if ($searchQuery === '' && $dateLimitUNIX && $dateLimitUNIX.start === '' && $filetypeShown === 'any') {
			triggerSearch();
		}
	});
</script>

<div class="page" in:fade>
	<SearchFilters />
	<Separator class="my-2 "/>
	<div class="flex flex-col justify-between max-h-[90%] h-full">
		<ResultsTable />
		<!-- {#if $documentsShown.length > 0 || $noMoreResults === true}
			<LoadMoreButton />
		{/if} -->
	</div>
</div>

<style>
	.page {
		overflow: hidden !important;
		position: relative;
		width: 100%;
		height: 100%;
	}
</style>
