<script lang="ts">
  import FiletypeDropdown from './FiletypeDropdown.svelte';
  import FiletypeSelector from './FiletypeSelector.svelte';
  import DatePicker from './DatePicker.svelte';
	import LocationSelector from './LocationSelector.svelte';
	import ResultViewToggleBar from './ResultViewToggleBar.svelte';
	import Separator from '../ui/separator/separator.svelte';
</script>

<div class="flex flex-row overflow-x-auto gap-4 py-4 justify-items-center">
  <LocationSelector />
  <FiletypeSelector />
  <DatePicker startDate={true} />
  <DatePicker startDate={false} />
  <ResultViewToggleBar />
</div>

<style lang="scss">

</style>