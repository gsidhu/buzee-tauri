<script lang="ts">
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { sendEvent } from '../../utils/firebase';

  async function startSerialEventListener() {
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