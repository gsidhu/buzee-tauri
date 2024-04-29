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
	<div id="topbar-bg" class="w-100">
		<TopBar />
	</div>
  <div
		class="d-flex flex-column gap-3 justify-content-center align-items-center col-10 col-sm-8 mx-auto mb-5"
	>
		<div class="page-icon">
			<i class="bi bi-file-earmark" />
		</div>
		<h3>Filetype List</h3>
    <!-- <p class="text-center">Below is a list of support filetypes and categories</p> -->

    {#key $allowedExtensions.items }
      <FileTypeList />
    {/key}
  </div>
</div>