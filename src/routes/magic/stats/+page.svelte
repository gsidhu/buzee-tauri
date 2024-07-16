<script lang="ts">
  import { fade } from "svelte/transition";
	import UniqueDocumentProfile from "$lib/components/stats/UniqueDocumentProfile.svelte";
</script>

<div class="flex flex-col" in:fade={{ delay: 0, duration: 500 }}>
  <h3 class="text-lg font-semibold leading-none tracking-tight">Document Stats</h3>
  <p class="text-sm text-muted-foreground">A data fan we have here, do we?</p>
</div>
<div class="flex flex-1 flex-col w-full items-center justify-center rounded-lg border border-dashed shadow-sm">
  <UniqueDocumentProfile />
</div>