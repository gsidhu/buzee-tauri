<script lang="ts">
  import { fade } from "svelte/transition";
  import TopBar from "../../layout/TopBar.svelte";
	import UniqueDocumentProfile from "$lib/components/stats/UniqueDocumentProfile.svelte";
</script>

<div in:fade={{ delay: 0, duration: 500 }}>
  <div id="topbar-bg" class="w-100">
    <TopBar />
  </div>
  <div class="d-flex flex-column col-10 col-sm-8 mx-auto mb-5">
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