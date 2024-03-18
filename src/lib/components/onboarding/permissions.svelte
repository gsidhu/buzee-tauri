<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from 'svelte';
  import { fly } from 'svelte/transition';
  import { dbCreationInProgress } from '$lib/stores';
  import ConfettiButton from "../ui/confettiButton.svelte";
	import Waiting from './waiting.svelte';

  let showWaiting = false;
  function startFileScan() {
    $dbCreationInProgress = true;
    invoke("run_file_indexing")
    showWaiting = true;
	}

  onMount(() => {
  })
</script>

<div in:fly={{ delay: 200, y: 100 }}>
  {#if showWaiting}
    <Waiting />
  {:else}
    <h3>Allow us to Scan Your Computer</h3>
    <p>We will need your permission to scan your Documents, Downloads and Desktop folders.</p>
    <p>You can always add more folders from the Settings in the app.</p>

    <h6 class="mt-4">Why Do We Need To Scan?</h6>
    <p class="small">Scanning helps us tailor Buzee to your unique use-case. The scanned results remain on your computer. We do not transmit any of your data to our servers.</p>

    <div class="text-center my-5">
      <ConfettiButton label="Start Scan" handleClick={startFileScan}/>
    </div>
  {/if}
</div>

<style lang="scss">
</style>