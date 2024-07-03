<script lang="ts">
  import { onMount } from "svelte";
  import { FoldVertical, LayoutGrid, List, UnfoldVertical } from "lucide-svelte";
  import Button from "../ui/button/button.svelte";
  import { Toggle } from "$lib/components/ui/toggle";
  import { Label } from "$lib/components/ui/label/index.js";
  import { compactViewMode, showIconGrid } from '$lib/stores';
  import { trackEvent } from '@aptabase/web';
	import Separator from "../ui/separator/separator.svelte";

  let toggleGroupValue: string[] = [];

  function setToggleValues() {
    $compactViewMode = toggleGroupValue.includes("compact") ? true : false;
    $showIconGrid = toggleGroupValue.includes("grid") ? true : false;
  }

  function toggleCompactViewMode() {
		$compactViewMode = !$compactViewMode;
		trackEvent('click:toggleCompactViewMode', { compactViewMode: $compactViewMode });
		if ($compactViewMode === true) {
			document.querySelectorAll('td').forEach((el) => {
				el.classList.add('compact-view');
			});
			document.querySelectorAll('th').forEach((el) => {
				el.classList.add('compact-view');
			});
		} else {
			document.querySelectorAll('td').forEach((el) => {
				el.classList.remove('compact-view');
			});
			document.querySelectorAll('th').forEach((el) => {
				el.classList.remove('compact-view');
			});
		}
	}

  onMount(() => {
    toggleGroupValue = $compactViewMode ? ["compact", ...toggleGroupValue] : [...toggleGroupValue];
    toggleGroupValue = $showIconGrid ? ["grid", ...toggleGroupValue] : [...toggleGroupValue];
  });
</script>

<div class="flex gap-4">
  <Separator class="hidden md:block w-[1px]" orientation="vertical" />
  <div class="flex flex-col">
    <Label class="mb-2">Switch View</Label>
    <div class="flex items-center gap-2">
        <Toggle
          variant="outline"
          class="gap-2 text-muted-foreground font-normal"
          title="Switch to Icon Grid"
          on:click={() => $showIconGrid = !$showIconGrid}
          pressed={$showIconGrid}
        >
          <LayoutGrid class="h-4 w-4" />Icons
      </Toggle>
      <Toggle
        variant="outline"
        class="gap-2 text-muted-foreground font-normal"
        title="Show results in compact view"
        on:click={() => toggleCompactViewMode()}
        pressed={$compactViewMode}
      >
        <FoldVertical class="h-4 w-4" />Tight
      </Toggle>
    </div>
  </div>
</div>