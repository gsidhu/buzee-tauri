<script lang="ts">
  import { extractDate } from "$lib/utils/queryParsing";
  import * as Command from "$lib/components/ui/command/index.js";
  import * as Popover from "$lib/components/ui/popover/index.js";
  import { Button } from "$lib/components/ui/button/index.js";

  let inputValue = "";
  let dateLimitUNIX:any;
  let dateLimitHuman:any;
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
    dateLimitUNIX = extractDate(value);
    if (dateLimitUNIX !== null) {
      dateLimitHuman = {start: new Date(dateLimitUNIX.start*1000).toLocaleDateString(), end: new Date(dateLimitUNIX.end*1000).toLocaleDateString()};
    } else {
      dateLimitHuman = null;
    }
  }
</script>
 

<Popover.Root bind:open let:ids>
  <Popover.Trigger asChild let:builder>
    <Button
      builders={[builder]}
      variant="outline"
      id="nlp-date-range-trigger"
      class="max-w-[250px] w-full justify-start flex flex-row items-start text-popover-foreground font-normal"
    >
      {#if inputValue === ""} Anytime
      {:else} {dateLimitHuman ? getDateLimitTextRange() : inputValue}
      {/if}
    </Button>
  </Popover.Trigger>
  <Popover.Content class="max-w-[250px] p-0" align="start">
    <Command.Root
      id="nlp-date-range-combobox"
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
        <Command.Group heading="Suggestions">
          <Command.Item onSelect={(v) => {parseInput("");}}>Anytime</Command.Item>
          <Command.Item onSelect={(v) => {parseInput(v);}}>Last Week</Command.Item>
          <Command.Item onSelect={(v) => {parseInput(v);}}>Last Year</Command.Item>
          <Command.Item onSelect={(v) => {parseInput(v);}}>This Month</Command.Item>
          <Command.Item onSelect={(v) => {parseInput(v);}}>This Year</Command.Item>
          <Command.Item onSelect={(v) => {parseInput(v);}}>From Jan 2023 to Aug 2023</Command.Item>
          <Command.Item onSelect={(v) => {parseInput(v);}}>Q1 2024</Command.Item>
        </Command.Group>
      </Command.List>
    </Command.Root>
  </Popover.Content>
</Popover.Root>