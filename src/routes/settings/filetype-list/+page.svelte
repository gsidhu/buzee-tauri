<script lang="ts">
  import { fade } from 'svelte/transition';
	import { onMount } from 'svelte';
  import { allowedExtensions } from '$lib/stores';
	import FileTypeList from '$lib/components/settings/fileTypeList.svelte';

	onMount(() => {
	});

</script>

<div class="flex flex-col" in:fade={{ delay: 0, duration: 500 }}>
  <h3 class="text-lg font-semibold leading-none tracking-tight">Filetype List</h3>
  <p class="text-sm text-muted-foreground">You will be able to modify this list in an upcoming update!</p>
</div>
<div class="flex flex-1 flex-col gap-2 items-center justify-between rounded-lg border border-dashed shadow-sm p-4">
	{#key $allowedExtensions.items }
		<FileTypeList />
	{/key}
</div>