<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { trackEvent } from '@aptabase/web';
  import { isMac, selectedResult, documentsShown, statusMessage, disableInteraction, syncStatus, selectedResultText } from '$lib/stores';
  import { confirm } from '@tauri-apps/plugin-dialog';

  async function startSerialEventListener() {
    // Result Row
    await listen<Payload>('open', (event: any) => {
      trackEvent('openFileOrFolder');
      invoke('open_file_or_folder', { filePath: $selectedResult.path });
    });
    await listen<Payload>('open-folder', (event: any) => {
      trackEvent('openParentFolder');
      invoke('open_folder_containing_file', { filePath: $selectedResult.path });
    });
    await listen<Payload>('ignore-item', async (event: any) => {      
      const fileOrFolder = $selectedResult.file_type === 'folder' ? 'folder' : 'file';
      let result: boolean = false;
      if ($isMac) {
        result = await confirm(
          `${$selectedResult.path}\n\nThis ${fileOrFolder === "folder" ? "folder and its contents" : "file"} will no longer show in Buzee's search results. You can always change this in the Settings.`, {
            title: `Are you sure you want Buzee to ignore this ${fileOrFolder}?`,
            okLabel: 'Yes',
            cancelLabel: 'No'
        });
      } else {
        result = await confirm(
          `Are you sure you want Buzee to ignore this ${fileOrFolder}?\n\n${$selectedResult.path}\n\nThis ${fileOrFolder === "folder" ? "folder and its contents" : "file"} will no longer show in Buzee's search results. You can always change this in the Settings.`, {
            title: `Ignore this ${fileOrFolder}?`,
            okLabel: 'Yes',
            cancelLabel: 'No'
        });
      }
      if (result) {
        trackEvent(`ignore:${fileOrFolder}`);
        $statusMessage = `Removing ${fileOrFolder}... Please wait.`;
        $syncStatus = true;
        $disableInteraction = true;
        invoke('ignore_file_or_folder', { path: $selectedResult.path, isDirectory: fileOrFolder === "folder" ? true : false, shouldIgnoreIndexing: true, shouldIgnoreContent: true }).then(() => {
          $statusMessage = `Removed!`;
          $syncStatus = false;
          $disableInteraction = false;
          setTimeout(() => {
            $statusMessage = '';
          }, 2000);
        });
        // remove $selectedResult from $documentsShown
        documentsShown.update((docs) => docs.filter(doc => doc.path !== $selectedResult.path));
      }
      else return;
    });
    await listen<Payload>('ignore-folder', async (event: any) => {
      const fileOrFolder = $selectedResult.file_type === 'folder' ? 'folder' : 'file';
      let parentFolder = '';
      if ($selectedResult.path.includes('/')) parentFolder = $selectedResult.path.split('/').slice(0, -1).join('/');
      else if ($selectedResult.path.includes('\\')) parentFolder = $selectedResult.path.split('\\').slice(0, -1).join('\\');

      let result: boolean = false;
      if ($isMac) {
        result = await confirm(
          `${parentFolder}\n\nThe contents of this folder will no longer show in Buzee's search results. You can always change this in the Settings.`, {
            title: `Are you sure you want Buzee to ignore this ${fileOrFolder}'s parent folder?`,
            okLabel: 'Yes',
            cancelLabel: 'No'
        });
      } else {
        result = await confirm(
          `Are you sure you want Buzee to ignore this ${fileOrFolder}'s parent folder?\n\n${parentFolder}\n\nThe contents of this folder will no longer show in Buzee's search results. You can always change this in the Settings.`, {
            title: `Ignore this ${fileOrFolder}'s parent folder?`,
            okLabel: 'Yes',
            cancelLabel: 'No'
        });
      }
      if (result) {
        trackEvent(`ignore:${fileOrFolder}-parent`);
        $statusMessage = `Removing ${fileOrFolder}... Please wait.`;
        $syncStatus = true;
        $disableInteraction = true;
        invoke('ignore_file_or_folder', { path: parentFolder, isDirectory: fileOrFolder === "folder" ? true : false, shouldIgnoreIndexing: true, shouldIgnoreContent: true }).then(() => {
          $statusMessage = `Removed!`;
          $syncStatus = false;
          $disableInteraction = false;
          setTimeout(() => {
            $statusMessage = '';
          }, 2000);
        });
        // remove $selectedResult from $documentsShown
        documentsShown.update((docs) => docs.filter(doc => doc.path !== $selectedResult.path));
      }
      else return;
    });
    await listen<Payload>('ignore-text', async (event: any) => {
      const fileOrFolder = $selectedResult.file_type === 'folder' ? 'folder' : 'file';
      let parentFolder = '';
      if ($selectedResult.path.includes('/')) parentFolder = $selectedResult.path.split('/').slice(0, -1).join('/');
      else if ($selectedResult.path.includes('\\')) parentFolder = $selectedResult.path.split('\\').slice(0, -1).join('\\');
      
      let result: boolean = false;
      if ($isMac) {
        result = await confirm(
          `${parentFolder}\n\nThe text of ${fileOrFolder === "folder" ? "all files in this folder" : "this file"} will no longer be scanned by Buzee. You can always change this in the Settings.`, {
            title: `Are you sure you want Buzee to ignore the text of ${fileOrFolder === "folder" ? "all files in this folder" : "this file"}?`,
            okLabel: 'Yes',
            cancelLabel: 'No'
        });
      } else {
        result = await confirm(
          `Are you sure you want Buzee to ignore the text of ${fileOrFolder === 'folder' ? "all files in this folder" : "this file"}?\n\n${parentFolder}\n\nThe text of ${fileOrFolder === "folder" ? "all files in this folder" : "this file"} will no longer be scanned by Buzee. You can always change this in the Settings.`, {
            title: `Ignore text?`,
            okLabel: 'Yes',
            cancelLabel: 'No'
        });
      }
      if (result) {
        trackEvent(`ignore:${fileOrFolder}-text`);
        $statusMessage = `Removing ${fileOrFolder}... Please wait.`;
        $syncStatus = true;
        $disableInteraction = true;
        invoke('ignore_file_or_folder', { path: parentFolder, isDirectory: fileOrFolder === "folder" ? true : false, shouldIgnoreIndexing: false, shouldIgnoreContent: true }).then(() => {
          $statusMessage = `Removed!`;
          $syncStatus = false;
          $disableInteraction = false;
          setTimeout(() => {
            $statusMessage = '';
          }, 2000);
        });
      }
      else return;
    });
    await listen<Payload>('show-preview', (event: any) => {
      trackEvent('showPreview');
      invoke('open_quicklook', { filePath: $selectedResult.path });
    });
    await listen<Payload>('show-text', (event: any) => {
      trackEvent('showText');
      if ($selectedResult !== undefined) {
        // @ts-ignore
        let textModal = new bootstrap.Modal(document.getElementById('file-text-modal'), {});
        invoke('get_text_for_file', { filePath: $selectedResult.path }).then((res) => {
          $selectedResultText = [];
          // @ts-ignore
          res.forEach(element => {
            $selectedResultText.push(element);
          });
        })
        textModal.show();
      }
    });
    // Status Bar - Fun Stuff
    await listen<Payload>('document-stats', (event: any) => {
      trackEvent('magic:stats');
      goto("/magic/stats");
    });
    await listen<Payload>('deep-breathing', (event: any) => {
      trackEvent('magic:deep-breathing');
      goto("/magic/deep-breathing");
    });
    await listen<Payload>('tips-and-shortcuts', (event: any) => {
      trackEvent('magic:tips');
      goto("/magic/tips");
    });
	}

  onMount(async () => {
    startSerialEventListener();
  });
</script>