<script lang="ts">
  import FiletypeSelector from './FiletypeSelector.svelte';
	import LocationSelector from './LocationSelector.svelte';
	import ResultViewToggleBar from './ResultViewToggleBar.svelte';
	import Separator from '../ui/separator/separator.svelte';
	import NlpDateRange from './NLPDateRange.svelte';
</script>

<div class="flex flex-row overflow-x-auto gap-4 py-4 justify-items-center">
<!-- <div class="grid grid-cols-5 gap-4 py-4 justify-items-stretch grid-flow-row overflow-x-auto max-w-[90vw]"> -->
  <LocationSelector />
  <FiletypeSelector />
  <NlpDateRange />
  <ResultViewToggleBar />
</div>

<style lang="scss">

</style>