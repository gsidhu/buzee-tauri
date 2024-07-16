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
	import { userPreferences } from '$lib/stores';

  let processingDone = false;
  let showPermissions = false;

  function gotoSearch() {
    $userPreferences.onboarding_done = true;
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

<div class="container my-10 h-full w-full px-4 lg:max-w-2xl">
  {#if !showPermissions}
    <div in:fly={{ y: 20 }} class="">
      <div class="text-center my-10">
        <img src="Buzee Logo.png" alt="Buzee Logo" width="96" height="96"/>
      </div>
      <h1>Welcome to Buzee!</h1>
      <p>Buzee helps you find your documents, effortlessly</p>

      <div class="text-center my-10">
        <ConfettiButton label="Get started!" handleClick={() => showPermissions = true} />
      </div>
    </div>
  {:else}
    <Permissions />
  {/if}
  <div id="processing-complete" class={`mt-5 text-center small ${processingDone ? 'faded' : 'hidden'} `} in:fade={{ delay: 200, duration: 1000 }}>
    <p class="my-5">Buzee is ready 💪</p>
    <button type="button" class="btn py-1 px-2 leading-tight text-xs my-2 text-dark border-2 border-dark rounded border-hover-purple" on:click={() => gotoSearch()}>
      Start searching!
    </button>
    <p class="small-explanation my-2">Note: Once the first full background scan <i class="fw-bold bi bi-arrow-repeat"/> finishes, you will be able to search keywords <em>inside</em> documents!<br/>The background scan runs automatically twice an hour. You can manually start/stop it using the <i class="fw-bold bi bi-arrow-repeat"/> button.</p>
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