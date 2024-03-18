<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import type { UnlistenFn } from '@tauri-apps/api/event';
  import { fly, fade } from 'svelte/transition';
	import ConfettiButton from "../ui/confettiButton.svelte";
	import Permissions from "./permissions.svelte";
	import { onDestroy, onMount } from 'svelte';

  let processingDone = false;
  let showPermissions = false;

  let unlisten:UnlistenFn;
  onMount(async () => {
    unlisten = await listen<Payload>('files-added', (event: any) => {
      if (event.payload.message === "files_added_complete") {
			  processingDone = true;
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

      <div class="col-4 mx-auto d-flex flex-column gap-3 my-5">
        <ConfettiButton label="Get started!" handleClick={() => showPermissions = true} />
      </div>
    </div>
  {:else}
    <Permissions />
  {/if}
  <div id="processing-complete" class={`mt-5 text-center small ${processingDone ? 'faded' : 'hidden'} `} in:fade={{ delay: 200, duration: 1000 }}>
    Buzee is ready. <a class="link-dark" href="/search?highlight-search-bar=true&q=last%20month">Start searching!</a>
  </div>
</div>

<style lang="scss">
  .hidden {
    opacity: 0;
  }
  .faded {
    opacity: 0.5;
  }
</style>