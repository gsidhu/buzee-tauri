<script lang="ts">
  import CalendarIcon from "lucide-svelte/icons/calendar";
  import {
    type DateValue,
    DateFormatter,
    getLocalTimeZone,
  } from "@internationalized/date";
  import { cn } from "$lib/utils.js";
  import { Button } from "$lib/components/ui/button";
  import { Calendar } from "$lib/components/ui/calendar";
  import * as Popover from "$lib/components/ui/popover";
  import * as Command from "$lib/components/ui/command/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { type ComponentType, tick } from "svelte";
 
  export let startDate = true;

  const todayDate = new Date();

  const df = new DateFormatter("en-US", {
    dateStyle: "long",
  });
 
  // let value: DateValue | undefined = undefined;

  let open = false;
  let value = {
    "day": "",
    "month": "",
    "year": ""
  };

  function checkInput() {
    console.log("checking");
    
    if (value.day.length > 2) {
      value.day = value.day.slice(0,2)
    }
    if (parseInt(value.day) > 31) {
      value.day = "31"
    }
    if (value.month !== "" && parseInt(value.day) > monthValues[parseInt(value.month) - 1].max) {
      if (parseInt(value.month) === 2) {
        if (value.year !== "" && parseInt(value.year) % 4 === 0) {
          value.day = "29";
        } else {
          value.day = "28";
        }
      } else {
        value.day = monthValues[parseInt(value.month) - 1].max.toString();
      }
    }
    if (value.year.length > 4) {
      value.year = value.year.slice(0,4)
    }
    if (parseInt(value.year) > todayDate.getFullYear()) {
      value.year = todayDate.getFullYear().toString();
    }
  }

  const monthValues = [
    {
      value: "1",
      label: "January",
      shortLabel: "Jan",
      max: 31
    },
    {
      value: "2",
      label: "February",
      shortLabel: "Feb",
      max: 29
    },
    {
      value: "3",
      label: "March",
      shortLabel: "Mar",
      max: 31
    },
    {
      value: "4",
      label: "April",
      shortLabel: "Apr",
      max: 30
    },
    {
      value: "5",
      label: "May",
      shortLabel: "May",
      max: 31
    },
    {
      value: "6",
      label: "June",
      shortLabel: "Jun",
      max: 30
    },
    {
      value: "7",
      label: "July",
      shortLabel: "Jul",
      max: 31
    },
    {
      value: "8",
      label: "August",
      shortLabel: "Aug",
      max: 31
    },
    {
      value: "9",
      label: "September",
      shortLabel: "Sep",
      max: 30
    },
    {
      value: "10",
      label: "October",
      shortLabel: "Oct",
      max: 31
    },
    {
      value: "11",
      label: "November",
      shortLabel: "Nov",
      max: 30
    },
    {
      value: "12",
      label: "December",
      shortLabel: "Dec",
      max: 31
    },
  ];

  $: selectedStatus = monthValues.find((s) => s.value === value.month) ?? null;
 
  // We want to refocus the trigger button when the user selects
  // an item from the list so users can continue navigating the
  // rest of the form with the keyboard.
  function closeAndFocusTrigger(triggerId: string) {
    open = false;
    tick().then(() => {
      document.getElementById(triggerId)?.focus();
    });
  }
</script>

<div class="flex flex-col w-full">
  <Label class="mb-2">
    {#if startDate}
      Start Date
    {:else}
      End Date
    {/if}
  </Label>
  <div class="">
    <Button 
      variant="outline"
      class="w-full justify-center flex flex-row items-start"
    >
      <input id={startDate ? "start-date-input" : "end-date-input"} bind:value={value.day} on:input={() => checkInput()} type="text" min=0 max=31 placeholder="DD" class="max-w-xs p-0 border-0 w-[2em] h-5" />
      <Popover.Root bind:open let:ids>
        <Popover.Trigger asChild let:builder>
          <Button
            builders={[builder]}
            variant="link"
            size="sm"
            class="w-[60px] justify-start h-5 text-muted-foreground"
          >
            {#if selectedStatus}
              {selectedStatus.shortLabel}
            {:else}
              MM
            {/if}
          </Button>
        </Popover.Trigger>
        <Popover.Content class="w-auto p-0">
          <Command.Root>
            <Command.Input class="calendar" placeholder="Month" />
            <Command.List>
              <Command.Empty>No results found.</Command.Empty>
              <Command.Group>
                {#each monthValues as status}
                  <Command.Item
                    value={status.value}
                    onSelect={(currentValue) => {
                      value.month = currentValue;
                      closeAndFocusTrigger(ids.trigger);
                      checkInput();
                    }}
                  >
                    <span>
                      {status.label}
                    </span>
                  </Command.Item>
                {/each}
              </Command.Group>
            </Command.List>
          </Command.Root>
        </Popover.Content>
      </Popover.Root>
      <input id={startDate ? "start-year-input" : "end-year-input"} bind:value={value.year} on:input={() => checkInput()} type="text" min=1900 max=2100 placeholder="YYYY" class="max-w-xs p-0 border-0 w-[3em]" />
    </Button>
  </div>
</div>