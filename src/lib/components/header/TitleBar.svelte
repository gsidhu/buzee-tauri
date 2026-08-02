<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import MenuBar from "./MenuBar.svelte";

  const win = getCurrentWindow();

  function minimize() {
    win.minimize();
  }

  function toggleMaximize() {
    win.toggleMaximize();
  }

  function close() {
    win.close();
  }
</script>

<div
  class="flex h-9 w-full shrink-0 items-stretch overflow-visible border-b bg-muted/40"
  data-tauri-drag-region
>
  <MenuBar />
  <div class="flex h-full flex-1 items-center gap-2 px-3" data-tauri-drag-region>
    <img src="/Buzee Logo.png" alt="Buzee" class="h-4 w-4" data-tauri-drag-region />
    <span class="text-sm font-semibold tracking-tight text-foreground" data-tauri-drag-region>
      Buzee
    </span>
  </div>
  <div class="flex h-full shrink-0 items-stretch">
    <button
      type="button"
      aria-label="Minimize"
      title="Minimize"
      on:click={minimize}
      class="flex h-9 w-11 cursor-default items-center justify-center text-muted-foreground outline-none hover:bg-accent"
    >
      <svg width="12" height="12" viewBox="0 0 12 12" aria-hidden="true">
        <rect x="1" y="5.5" width="10" height="1" fill="currentColor"></rect>
      </svg>
    </button>
    <button
      type="button"
      aria-label="Maximize"
      title="Maximize"
      on:click={toggleMaximize}
      class="flex h-9 w-11 cursor-default items-center justify-center text-muted-foreground outline-none hover:bg-accent"
    >
      <svg width="11" height="11" viewBox="0 0 12 12" aria-hidden="true">
        <rect x="1.5" y="1.5" width="9" height="9" fill="none" stroke="currentColor" stroke-width="1"></rect>
      </svg>
    </button>
    <button
      type="button"
      aria-label="Close"
      title="Close"
      on:click={close}
      class="flex h-9 w-11 cursor-default items-center justify-center text-muted-foreground outline-none hover:bg-[#c42b1c] hover:text-white"
    >
      <svg width="11" height="11" viewBox="0 0 12 12" aria-hidden="true">
        <path
          d="M1 1 L11 11 M11 1 L1 11"
          fill="none"
          stroke="currentColor"
          stroke-width="1.2"
        ></path>
      </svg>
    </button>
  </div>
</div>