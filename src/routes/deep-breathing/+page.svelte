<script lang="ts">
  import { fade } from "svelte/transition";
  import TopBar from "../../layout/TopBar.svelte";
</script>

<div in:fade={{ delay: 0, duration: 500 }}>
  <div id="topbar-bg" class="w-100">
    <TopBar />
  </div>
  <div class="d-flex flex-column col-10 col-sm-8 mx-auto mb-5">
    <div class="text-center gap-2">
      <div class="page-icon">
        <i class="bi bi-yin-yang"></i>
      </div>
      <h3>Take A Deep Breath</h3>
      <p>Just follow the animation</p>
      <div id="breathing" class="mx-auto mt-5"></div>
    </div>
  </div>
</div>

<style>
  #breathing {
		background-image: linear-gradient(to top, #fad0c4 0%, #ffd1ff 100%);
		width: 8rem;
		height: 8rem;
		border-radius: 67% 33% 63% 37% / 46% 32% 68% 54%;
    animation: breathing-circle 10s linear infinite;
	}
</style>