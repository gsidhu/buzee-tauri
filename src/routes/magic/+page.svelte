<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { goto } from "$app/navigation";
	import { trackEvent } from '@aptabase/web';
  import TopBar from "../../layout/TopBar.svelte";

	async function openDeadlineInBrowser() {
		trackEvent("magic:deadline");
		await invoke("open_file_or_folder", { filePath: "https://buzo.tools/deadline/" });
	}

	function gotoMagicPage(page: string) {
		trackEvent(`magic:${page}`);
		goto(`/magic/${page}`);
	}

  onMount(() => {
		trackEvent("magic:homepage");
  })

</script>

<div in:fade={{ delay: 0, duration: 500 }}>
  <div id="topbar-bg" class="w-100">
    <TopBar />
  </div>
  <div
		class="d-flex flex-column gap-3 justify-content-center align-items-center col-10 col-sm-8 mx-auto mb-5"
	>
    <div class="page-icon">
      <i class="bi bi-stars"></i>
    </div>
    <h3>Fun Stuff</h3>
    <table class="table table-bordered w-90 mb-0">
			<tr>
				<td class="text-center px-2">
					<button class="btn" on:click={() => openDeadlineInBrowser()}>
            <i class="bi bi-trophy" />
					</button>
				</td>
				<td class="py-2" role="button" on:click={() => openDeadlineInBrowser()}>
					Deadline<i class="bi bi-box-arrow-up-right skip-icon"></i>
					<div class="d-flex align-items-center small-explanation gap-1">
						<div>A gentle progress tracker made with kindness</div>
					</div>
				</td>
			</tr>
			<tr>
				<td class="text-center px-2">
					<button class="btn" on:click={() => gotoMagicPage('deep-breathing')}>
            <i class="bi bi-yin-yang" />
					</button>
				</td>
				<td class="py-2" role="button" on:click={() => gotoMagicPage('deep-breathing')}>
					Deep Breathing
					<div class="d-flex align-items-center small-explanation gap-1">
						<div>Take a few moments to yourself</div>
					</div>
				</td>
			</tr>
			<tr>
				<td class="text-center px-2">
					<button class="btn" on:click={() => gotoMagicPage('stats')}>
            <i class="bi bi-pie-chart" />
					</button>
				</td>
				<td class="py-2" role="button" on:click={() => gotoMagicPage('stats')}>
					Document Stats
					<div class="d-flex align-items-center small-explanation gap-1">
						<div>See what your Unique Document Profile looks like</div>
					</div>
				</td>
			</tr>
			<tr>
				<td class="text-center px-2">
					<button class="btn" on:click={() => gotoMagicPage('extract-text')}>
            <i class="bi bi-body-text" />
					</button>
				</td>
				<td class="py-2" role="button" on:click={() => gotoMagicPage('extract-text')}>
					Extract Text from PDF or Image
					<div class="d-flex align-items-center small-explanation gap-1">
						<div>Extract text from any kind of PDF or image file</div>
					</div>
				</td>
			</tr>
			<tr>
				<td class="text-center px-2">
					<button class="btn" on:click={() => gotoMagicPage('scratchpad')}>
            <i class="bi bi-journal-text" />
					</button>
				</td>
				<td class="py-2" role="button" on:click={() => gotoMagicPage('scratchpad')}>
					Scratch Pad
					<div class="d-flex align-items-center small-explanation gap-1">
						<div>A place to keep copied text, notes, and thoughts</div>
					</div>
				</td>
			</tr>
			<tr>
				<td class="text-center px-2">
					<button class="btn" on:click={() => gotoMagicPage('tips')}>
            <i class="bi bi-lightbulb" />
					</button>
				</td>
				<td class="py-2" role="button" on:click={() => gotoMagicPage('tips')}>
					Tips & Shortcuts
					<div class="d-flex align-items-center small-explanation gap-1">
						<div>Read some hints to get the best out of Buzee</div>
					</div>
				</td>
			</tr>
		</table>
  </div>
</div>

<style lang="scss">
  .small-explanation {
		font-size: 0.7rem;
		font-weight: 300;
		padding: 0;
		background-color: inherit;
		color: var(--bs-gray);
	}

	tr:hover {
    cursor: default;
    color: var(--purple);
	}

	i:not(.skip-icon) {
		font-size: 1.5rem;
	}

	i.skip-icon {
		font-size: 0.8rem;
	}
</style>