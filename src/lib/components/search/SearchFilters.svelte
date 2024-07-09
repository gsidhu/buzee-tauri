<script lang="ts">
  import FiletypeSelector from './FiletypeSelector.svelte';
	import LocationSelector from './LocationSelector.svelte';
	import ResultViewToggleBar from './ResultViewToggleBar.svelte';
	import NlpDateRange from './NLPDateRange.svelte';
  import ChevronsUpDown from "lucide-svelte/icons/chevrons-up-down";
  import * as Collapsible from "$lib/components/ui/collapsible/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
	import ResultsPerPageSelector from './ResultsPerPageSelector.svelte';
  import { searchFiltersOpen } from "$lib/stores";
</script>

<Collapsible.Root class="space-y-2" bind:open={$searchFiltersOpen}>
  <div class="flex items-center justify-start space-x-4 px-4">
    <Collapsible.Trigger asChild let:builder>
      <Button builders={[builder]} variant="link" class="gap-1 p-0">
        <Label class="font-medium text-sm hover:cursor-pointer">Filter</Label>
        <ChevronsUpDown class="h-3 w-3" />
        <span class="sr-only">Toggle</span>
      </Button>
    </Collapsible.Trigger>
  </div>
  <Collapsible.Content class="">
    <div class="flex flex-row overflow-x-auto gap-4 px-4 pb-2 justify-items-center">
      <!-- <ResultsPerPageSelector /> -->
      <LocationSelector />
      <FiletypeSelector />
      <NlpDateRange />
      <ResultViewToggleBar />
    </div>
  </Collapsible.Content>
</Collapsible.Root>