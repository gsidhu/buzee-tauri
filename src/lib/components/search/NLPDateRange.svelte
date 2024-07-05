<script lang="ts">
  import { extractDate } from "$lib/utils/queryParsing";
  import * as Command from "$lib/components/ui/command/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import Label from "$lib/components/ui/label/label.svelte";
  import { searchQuery, dateLimitUNIX } from "$lib/stores";
  import { triggerSearch } from "$lib/utils/dbUtils";

  let inputValue = "";
  let localDateLimitUNIX:ParsedDatesUNIX | null;
  let dateLimitHuman:ParsedDatesUNIX | null;
  let open = false;

  $: inputValue.length > 0 ? open = true : open = false;

  $: if (inputValue.length > 3) {
    parseInput(inputValue);
  }

  function getDateLimitTextRange() {
    if (dateLimitHuman && dateLimitHuman.start && dateLimitHuman.end) {
      return dateLimitHuman.start + " - " + dateLimitHuman.end;
    } else {
      return "Custom";
    }
  }

  function parseInput(value: string) {
    inputValue = value;
    localDateLimitUNIX = extractDate(value);
    
    if (localDateLimitUNIX !== null) {
      $dateLimitUNIX = localDateLimitUNIX;
      dateLimitHuman = {
        start: new Date(parseInt(localDateLimitUNIX.start)*1000).toLocaleDateString(), 
        end: new Date(parseInt(localDateLimitUNIX.end)*1000).toLocaleDateString(),
        text: value
      };
    } else {
      $dateLimitUNIX.start = '';
      $dateLimitUNIX.end = '';
      $dateLimitUNIX.text = '';
      dateLimitHuman = null;
    }
  }

  async function triggerSearchLocal() {
    if (localDateLimitUNIX) {
      localDateLimitUNIX.text = $searchQuery;
      $dateLimitUNIX = localDateLimitUNIX;
    }
		(document.querySelector('button[data-dialog-close]') as HTMLElement)?.click();
		triggerSearch();
	}

  $: if ($dateLimitUNIX.start !== "") {
    console.log("nlp date range");
    dateLimitHuman = {
      start: new Date(parseInt($dateLimitUNIX.start)*1000).toLocaleDateString(), 
      end: new Date(parseInt($dateLimitUNIX.end)*1000).toLocaleDateString(),
      text: $dateLimitUNIX.text
    };
  }
</script>

<div class="flex flex-col w-full">
	<Label class="mb-2">
    Date Range
  </Label>
  <div class="flex flex-row gap-4 w-full">
    <Popover.Root bind:open let:ids>
      <Popover.Trigger asChild let:builder>
        <Button
          builders={[builder]}
          variant="outline"
          class="max-w-[250px] w-full justify-start flex flex-row items-start text-popover-foreground font-normal"
        >
          {#if inputValue === "" && $dateLimitUNIX.start === "" && dateLimitHuman === null} Anytime
          {:else} {dateLimitHuman ? getDateLimitTextRange() : (inputValue.length > 0 ? inputValue : "Anytime")}
          {/if}
        </Button>
      </Popover.Trigger>
      <Popover.Content class="max-w-[250px] p-0" align="start">
        <Command.Root
          filter={(value, search) => {
            if (value.includes(search) || search === inputValue) return 1;
            return 0;
          }}
        >
          <Command.Input 
            bind:value={inputValue} 
            placeholder="Anytime"
            class="h-9 calendar"
          />
          <Command.List>
            {#if inputValue.length > 0}
              <Command.Group heading="" class="h-[0px] p-0">
                <Command.Item class="text-[0px]" value={inputValue} onSelect={() => {parseInput(inputValue); open = false; triggerSearchLocal()}}>
                  {inputValue}
                </Command.Item>
              </Command.Group>
            {/if}
            <Command.Group heading="Suggestions">
              <Command.Item onSelect={(v) => {parseInput(""); open = false; triggerSearchLocal()}}>Anytime</Command.Item>
              <Command.Item onSelect={(v) => {parseInput(v); open = false; triggerSearchLocal()}}>Last Week</Command.Item>
              <Command.Item onSelect={(v) => {parseInput(v); open = false; triggerSearchLocal()}}>Last Year</Command.Item>
              <Command.Item onSelect={(v) => {parseInput(v); open = false; triggerSearchLocal()}}>This Month</Command.Item>
              <Command.Item onSelect={(v) => {parseInput(v); open = false; triggerSearchLocal()}}>This Year</Command.Item>
              <Command.Item onSelect={(v) => {parseInput(v); open = false; triggerSearchLocal()}}>From Jan to Aug 2023</Command.Item>
              <Command.Item onSelect={(v) => {parseInput(v); open = false; triggerSearchLocal()}}>Q1 2024</Command.Item>
            </Command.Group>
          </Command.List>
        </Command.Root>
      </Popover.Content>
    </Popover.Root>
  </div>
</div>