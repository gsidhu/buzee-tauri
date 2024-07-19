<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Check, RefreshCw } from "lucide-svelte";
  import { syncStatus, statusMessage, dbCreationInProgress } from "$lib/stores";
  import { trackEvent } from "@aptabase/web";
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { invoke } from '@tauri-apps/api/core';
  import * as ContextMenu from "$lib/components/ui/context-menu";

  let fileSyncFinished = false;
  let syncCoolingPeriod = false;
	let userAskedToDisable = false;

  async function toggleBackgroundTextProcessing() {
    trackEvent('click:toggleBackgroundTextProcessing', { syncStats: $syncStatus });
    // if $syncStatus is true, switch_off is true, so we want to stop the sync
    invoke("run_file_sync", {switchOff: $syncStatus, filePaths: []});
    if ($syncStatus) {
      $statusMessage = "Stopping background scan...";
      setTimeout(() => {$statusMessage = "";}, 3000);
      userAskedToDisable = true;
    } else {
      $statusMessage = "Starting background scan...";
      setTimeout(() => {$statusMessage = "";}, 3000);
    }
    // disable `bg-sync-btn` for 5 seconds
    // this allows any pending processes to complete when stopping the sync
    syncCoolingPeriod = true;
    setTimeout(() => {
      // if userAskedToDisable and sync is still running, then keep the cooling period on
      if (userAskedToDisable && $syncStatus) {
        syncCoolingPeriod = true;
      } else {
        syncCoolingPeriod = false;
      }
    }, 5000);
  }

  // if $syncStatus is false, then reset cooling period and userAskedToDisable
  $: if (!$syncStatus) {
    userAskedToDisable = false;
    syncCoolingPeriod = false;
  }

  // FOR SYNC STATUS WHEN CLICKED
	let unlisten_sync_status:UnlistenFn;
	// FOR FILE SYNC FINISHED
	let unlisten_file_sync_finished:UnlistenFn;

  onMount(async () => {
		// Listener for sync status changes from inside the Tokio process in db_sync.rs
		unlisten_sync_status = await listen<Payload>('sync-status', (event: any) => {
			$syncStatus = event.payload.data === 'true';
		});
		// Listener for when the db_sync process is done
		unlisten_file_sync_finished = await listen<Payload>('file-sync-finished', (event: any) => {
			fileSyncFinished = event.payload.data === 'true';
		});

		// Ask for sync status on each mount to keep it updated in case of page changes
		$syncStatus = await invoke("get_sync_status") === 'true';
	});

	onDestroy(() => {
		unlisten_sync_status();
		unlisten_file_sync_finished();
	});
</script>

<ContextMenu.Root>
  <ContextMenu.Trigger>
    <Button
      id="bg-sync-btn"
      variant={`${$syncStatus ? (syncCoolingPeriod ? 'secondary' : 'purple') : 'secondary'}`}
      size="icon"
      class={`px-2 rounded-full ${($syncStatus && syncCoolingPeriod) ? 'disabled-gray' : ''}`}
      on:click={() => toggleBackgroundTextProcessing()}
      disabled={syncCoolingPeriod}
      title={syncCoolingPeriod ? 
        `${userAskedToDisable ? "Please wait... Shutting down background processes" : "Booting up... Please wait for a few seconds"}` 
        : 
        `Click to ${$syncStatus ? 'stop' : 'start'} background scan`
      }
    >
      {#if $dbCreationInProgress && $syncStatus}
        <Check class="h-5 w-5" />
      {:else}
        <RefreshCw class={`h-5 w-5 ${$syncStatus ? 'spin-right' : ''}`} />
        <span class="sr-only">Toggle background sync</span>
      {/if}
    </Button>
  </ContextMenu.Trigger>
  <ContextMenu.Content>
    <ContextMenu.Item>View Sync Log</ContextMenu.Item>
  </ContextMenu.Content>
</ContextMenu.Root>