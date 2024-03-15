<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { sendEvent } from '../../utils/firebase';
  import { selectedResult } from '$lib/stores';

  async function startSerialEventListener() {
    // Result Row
    await listen<Payload>('open', (event: any) => {
      invoke('open_file_or_folder', { filePath: $selectedResult.path });
    });
    await listen<Payload>('open-folder', (event: any) => {
      invoke('open_folder_containing_file', { filePath: $selectedResult.path });
    });
    await listen<Payload>('show-preview', (event: any) => {
      invoke('open_quicklook', { filePath: $selectedResult.path });
    });
    // Status Bar - Fun Stuff
    await listen<Payload>('document-stats', (event: any) => {
      goto("/stats");
    });
    await listen<Payload>('deep-breathing', (event: any) => {
      goto("/deep-breathing");
    });
    await listen<Payload>('tips-and-shortcuts', (event: any) => {
      goto("/tips");
    });
	}

  onMount(async () => {
    startSerialEventListener();
  });
</script>