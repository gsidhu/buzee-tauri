<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from 'svelte';
  import { fly } from 'svelte/transition';
  import { dbCreationInProgress, userPreferences } from '$lib/stores';
  import ConfettiButton from "../ui/confettiButton.svelte";
	import Waiting from './waiting.svelte';
	import Button from "../ui/button/button.svelte";
  import { trackEvent } from '@aptabase/web';
  import { goto } from '$app/navigation';

  let showWaiting = false;
  
  function startFileScan() {
    $dbCreationInProgress = true;
    invoke("run_file_indexing", { filePaths: [], isFolder: false })
    showWaiting = true;
	}

  function manualSetup() {
    trackEvent("onboarding:manual");
    $userPreferences.onboarding_done = true;
    invoke("set_user_preference", {key: "manual_setup", value: true}).then(() => {
      console.log("Set manual setup flag to: " + true);
    });
    invoke("set_user_preference", {key: "onboarding_done", value: true}).then(() => {
      console.log("Set onboarding done flag to: " + true);
    });
    goto("/settings");
  }

  onMount(() => {
  })
</script>

<div in:fly={{ delay: 200, y: 100 }} class="my-10">
  {#if showWaiting}
    <div>
      <Waiting />
    </div>
  {:else}
    <h3>Allow Buzee to Scan Your Computer</h3>
    <p>We will need your permission to scan your computer.</p>
    <p class="small">The scanned results are stored in a database in your Documents folder. Your data never leaves your computer.</p>

    <div class="text-center my-10">
      <ConfettiButton label="Start Scan" handleClick={startFileScan}/>
    </div>

    <h3>Want to skip the automatic setup?</h3>
    <p>If you'd like to manually select the files and folders to add to Buzee, you can do that!</p>

    <div class="text-center my-10">
      <Button variant="outline" on:click={() => manualSetup()}>Manual Setup</Button>
    </div>
  {/if}
</div>

<style lang="scss">
</style>