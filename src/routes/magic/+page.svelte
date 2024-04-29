<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import TopBar from "../../layout/TopBar.svelte";
  let isMac = false;

  onMount(() => {
    invoke("get_os").then((res) => {
			// @ts-ignore
			if (res == "macos") {
				isMac = true;
			} else {
				isMac = false;
			}
		});
  })

</script>

<div in:fade={{ delay: 0, duration: 500 }}>
  <div id="topbar-bg" class="w-100">
    <TopBar />
  </div>
  <div class="d-flex flex-column col-10 col-sm-8 mx-auto mb-5">
    <div class="text-center gap-2">
      <div class="page-icon">
        <i class="bi bi-stars"></i>
      </div>
      <h3>The Fun Stuff</h3>
    </div>
    <a href="/magic/deep-breathing">Deep Breathing</a>
    <a href="/magic/stats">Document Stats</a>
    <a href="/magic/tips">Tips & Shortcuts</a>
  </div>
</div>