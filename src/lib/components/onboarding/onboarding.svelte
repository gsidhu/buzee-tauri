<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';
  import type { UnlistenFn } from '@tauri-apps/api/event';
  import { fly, fade } from 'svelte/transition';
	import ConfettiButton from "../ui/confettiButton.svelte";
	import Permissions from "./permissions.svelte";
  import { goto } from '$app/navigation';
	import { onDestroy, onMount } from 'svelte';
  import { trackEvent } from '@aptabase/web';

  let processingDone = false;
  let showPermissions = false;

  function gotoSearch() {
    goto("/search?highlight-search-bar=true&q=last%20month");
    trackEvent("onboarding:complete");
  }

  let unlisten:UnlistenFn;
  onMount(async () => {
    unlisten = await listen<Payload>('files-added', (event: any) => {
      if (event.payload.message === "files_added_complete") {
			  processingDone = true;
        invoke("set_user_preference", {key: "onboarding_done", value: true}).then(() => {
          console.log("Set onboarding done flag to: " + true);
        });
      }
		});
  })
  onDestroy(() => {
    unlisten();
  })
</script>

<div class="container px-4">
  {#if !showPermissions}
    <div in:fly={{ y: 20 }}>
      <div class="text-center my-2">
        <img src="Buzee Logo.png" alt="Buzee Logo" width="96" height="96"/>
      </div>
      <h1>Welcome to Buzee!</h1>
      <p>Buzee helps you find your documents, effortlessly</p>

      <div class="col-4 mx-auto flex flex-col gap-3 my-5">
        <ConfettiButton label="Get started!" handleClick={() => showPermissions = true} />
      </div>
    </div>
  {:else}
    <Permissions />
  {/if}
  <div id="processing-complete" class={`mt-5 text-center small ${processingDone ? 'faded' : 'hidden'} `} in:fade={{ delay: 200, duration: 1000 }}>
    <p class="mb-1">Buzee is ready</p>
    <button type="button" class="btn py-1 px-2 leading-tight text-xs my-2 text-dark border-2 border-dark rounded border-hover-purple" on:click={() => gotoSearch()}>
      Start searching!
    </button>
    <p class="small-explanation my-2">Note: Once the first full background scan <i class="fw-bold bi bi-arrow-repeat"/> finishes, you will be able to search keywords <em>inside</em> documents!<br/>The background scan runs automatically twice an hour. You can manually start/stop it using the button in the Status Bar below.</p>
  </div>
</div>

<style lang="scss">
  .hidden {
    opacity: 0;
  }
  .faded {
    opacity: 0.5;
  }

  .small-explanation {
		font-size: 0.7rem;
		font-weight: 300;
		padding: 0;
		background-color: inherit;
	}
</style>