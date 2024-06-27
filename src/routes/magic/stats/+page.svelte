<script lang="ts">
  import { fade } from "svelte/transition";
  import TopBar from "../../../layout/TopBar.svelte";
	import UniqueDocumentProfile from "$lib/components/stats/UniqueDocumentProfile.svelte";
</script>

<div in:fade={{ delay: 0, duration: 500 }}>
  <div id="topbar-bg" class="w-full">
    <TopBar />
  </div>
  <div class="flex flex-col w-4/5 pr-4 pl-4 sm:w-2/3 mx-auto mb-5">
    <div class="text-center gap-2">
      <div class="page-icon">
        <i class="bi bi-pie-chart"></i>
      </div>
      <h3>Document Stats</h3>
      <p>A data fan we have here, do we?</p>
    </div>
    <div class="">
      <UniqueDocumentProfile />
    </div>
  </div>
</div>