<script lang="ts">
  import { extractDate } from "$lib/utils/queryParsing";
  import * as Command from "$lib/components/ui/command/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import Label from "$lib/components/ui/label/label.svelte";
  import { searchQuery, resultsPageShown, searchInProgress, filetypeShown, resultsPerPage, documentsShown, allowedExtensions, base64Images, dateLimitUNIX } from "$lib/stores";
  import { trackEvent } from "@aptabase/web";
  import { searchDocuments, getDocumentsFromDB } from "$lib/utils/dbUtils";
  import { setExtensionCategory } from "$lib/utils/miscUtils";

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
      dateLimitHuman = null;
    }
  }

  async function triggerSearch() {
    if (localDateLimitUNIX) {
      localDateLimitUNIX.text = $searchQuery;
      $dateLimitUNIX = localDateLimitUNIX;
    }
		(document.querySelector('button[data-dialog-close]') as HTMLElement)?.click();
		$resultsPageShown = 0; // reset the page number on each new search
		$searchInProgress = true;
		$base64Images = {};
		trackEvent('search-triggered', {
			filetypeShown: $filetypeShown,
			resultsPageShown: $resultsPageShown
		});
		let filetypeToGet = $filetypeShown;
		if (filetypeToGet !== 'any') {
			filetypeToGet = setExtensionCategory($filetypeShown, $allowedExtensions);
		}
    console.log($searchQuery);
    
    $documentsShown = await searchDocuments(
      $searchQuery,
      $resultsPageShown,
      $resultsPerPage,
      filetypeToGet,
      localDateLimitUNIX
    );
		$searchInProgress = false;
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
          {#if inputValue === ""} Anytime
          {:else} {dateLimitHuman ? getDateLimitTextRange() : inputValue}
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
                <Command.Item class="text-[0px]" value={inputValue} onSelect={() => {parseInput(inputValue); open = false; triggerSearch()}}>
                  {inputValue}
                </Command.Item>
              </Command.Group>
            {/if}
            <Command.Group heading="Suggestions">
              <Command.Item onSelect={(v) => {parseInput(""); open = false; triggerSearch()}}>Anytime</Command.Item>
              <Command.Item onSelect={(v) => {parseInput(v); open = false; triggerSearch()}}>Last Week</Command.Item>
              <Command.Item onSelect={(v) => {parseInput(v); open = false; triggerSearch()}}>Last Year</Command.Item>
              <Command.Item onSelect={(v) => {parseInput(v); open = false; triggerSearch()}}>This Month</Command.Item>
              <Command.Item onSelect={(v) => {parseInput(v); open = false; triggerSearch()}}>This Year</Command.Item>
              <Command.Item onSelect={(v) => {parseInput(v); open = false; triggerSearch()}}>From Jan to Aug 2023</Command.Item>
              <Command.Item onSelect={(v) => {parseInput(v); open = false; triggerSearch()}}>Q1 2024</Command.Item>
            </Command.Group>
          </Command.List>
        </Command.Root>
      </Popover.Content>
    </Popover.Root>
  </div>
</div>