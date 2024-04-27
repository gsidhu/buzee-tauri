<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { sendEventToFirebase } from '../../utils/firebase';
  import { selectedResult } from '$lib/stores';
  import { confirm } from '@tauri-apps/plugin-dialog';

  async function startSerialEventListener() {
    // Result Row
    await listen<Payload>('open', (event: any) => {
      invoke('open_file_or_folder', { filePath: $selectedResult.path });
    });
    await listen<Payload>('open-folder', (event: any) => {
      invoke('open_folder_containing_file', { filePath: $selectedResult.path });
    });
    await listen<Payload>('ignore-item', async (event: any) => {
      console.log("pinging");
      
      const fileOrFolder = $selectedResult.file_type === 'folder' ? 'folder' : 'file';
      const result = await confirm(`${$selectedResult.path}\n\nThis ${fileOrFolder === "folder" ? "folder and its contents" : "file"} will no longer show in Buzee's search results. You can always change this in the Settings.`, {
        title: `Are you sure you want Buzee to ignore this ${fileOrFolder}?`,
        okLabel: 'Yes',
        cancelLabel: 'No'
      });
      if (result) invoke('ignore_file_or_folder', { filePath: $selectedResult.path });
      else return;
    });
    await listen<Payload>('ignore-folder', async (event: any) => {
      console.log("poingang");
      const fileOrFolder = $selectedResult.file_type === 'folder' ? 'folder' : 'file';
      let parentFolder = '';
      if ($selectedResult.path.includes('/')) parentFolder = $selectedResult.path.split('/').slice(0, -1).join('/');
      else if ($selectedResult.path.includes('\\')) parentFolder = $selectedResult.path.split('\\').slice(0, -1).join('\\');

      const result = await confirm(`${parentFolder}\n\nThe contents of this folder will no longer show in Buzee's search results. You can always change this in the Settings.`, {
        title: `Are you sure you want Buzee to ignore this ${fileOrFolder}'s parent folder?`,
        okLabel: 'Yes',
        cancelLabel: 'No'
      });
      if (result) invoke('ignore_file_or_folder', { filePath: parentFolder });
      else return;
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