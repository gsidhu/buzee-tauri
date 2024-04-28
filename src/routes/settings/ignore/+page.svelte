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
		<p>Any files or folders that you manually add from the Settings will be automatically removed from the Ignore List.</p>

		<button>Remove Item</button>
		<h6>SvelteTable: Ignore Completely</h6>
		<small>Dialog menu to add items</small>

		<h6>SvelteTable: Ignore Text</h6>
		<small>Dialog menu to add items</small>

		<button>Clear List</button>
		<button>Export List (to use in Buzee later)</button>
		<button>Import List (from a previous Buzee installation)</button>
		
		<small>On each save/import, run a formatting check + assign isFolder attribute before passing to the database</small>
  </div>
</div>