<script lang="ts">
  import { Button } from "$lib/components/ui/button";;
  import * as Popover from "$lib/components/ui/popover";
	import DatePicker from "./DatePicker.svelte";
  import Label from "$lib/components/ui/label/label.svelte";
	import NlpDateRange from "./NLPDateRange.svelte";

  let open = false;
</script>

<div class="flex flex-col w-full">
	<Label class="mb-2">
    Date Range
  </Label>
  <div class="flex flex-row gap-4 w-full">
    <NlpDateRange />
    <!-- <Popover.Root bind:open>
      <Popover.Trigger>
        <Button
          variant="outline"
          class="max-w-[200px] w-full justify-start flex flex-row items-start text-popover-foreground font-normal"
        >
          Select
        </Button>
      </Popover.Trigger>
      <Popover.Content class="w-auto p-0">
      <div class="flex flex-row w-full gap-4 p-4">
        <DatePicker startDate={true} />
        <DatePicker startDate={false} />
      </div>
      </Popover.Content>
    </Popover.Root>  -->
  </div>
</div>