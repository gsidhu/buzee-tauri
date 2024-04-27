<script lang="ts">
  import { fade } from 'svelte/transition';
  import TopBar from '../../../layout/TopBar.svelte';
	import PopoverIcon from '$lib/components/ui/popoverIcon.svelte';
	import { onMount } from 'svelte';

</script>

<div in:fade={{ delay: 0, duration: 500 }}>
	<div id="topbar-bg" class="w-100">
		<TopBar />
	</div>
  <div
		class="d-flex flex-column gap-3 justify-content-center align-items-center col-10 col-sm-8 mx-auto mb-5"
	>
		<div class="page-icon">
			<i class="bi bi-file-earmark-x" /> <i class="bi bi-folder-x" />
		</div>
		<h3>Ignore List</h3>
  </div>
</div>