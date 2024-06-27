<script lang="ts">
  import { fade } from 'svelte/transition';
  import TopBar from '../../../layout/TopBar.svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
  import { allowedExtensions } from '$lib/stores';
	import FileTypeList from '$lib/components/settings/fileTypeList.svelte';

	onMount(() => {
	});

</script>

<div in:fade={{ delay: 0, duration: 500 }}>
	<div id="topbar-bg" class="w-full">
		<TopBar />
	</div>
  <div
		class="flex flex-col gap-3 justify-content-center items-center w-4/5 pr-4 pl-4 sm:w-2/3 mx-auto mb-5"
	>
		<div class="page-icon">
			<i class="bi bi-file-earmark" />
		</div>
		<h3>Filetype List</h3>
    <p class="text-center">You will be able to modify this list in an upcoming update!</p>

    {#key $allowedExtensions.items }
      <FileTypeList />
    {/key}
  </div>
</div>