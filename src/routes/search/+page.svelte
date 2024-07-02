<script lang="ts">
	import { fade } from 'svelte/transition';
	import ResultsTable from '$lib/components/results/resultsTable.svelte';
	import SearchFilters from '$lib/components/search/SearchFilters.svelte';
	import Separator from '$lib/components/ui/separator/separator.svelte';
</script>

<div class="page" in:fade>
	<SearchFilters />
	<Separator class="my-2 "/>
	<div class="result-container">
		<ResultsTable />
	</div>
</div>

<style>
	.page {
		overflow: hidden !important;
		position: relative;
		width: 100%;
		height: 100%;
	}

	.result-container {
		height: 100%;
  	overflow: hidden !important;
	}
</style>
