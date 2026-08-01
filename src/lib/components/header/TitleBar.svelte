<script lang="ts">
  import { WindowTitlebar } from "@tauri-controls/svelte";
  import { isMac } from "$lib/stores";
  import MenuBar from "./MenuBar.svelte";
</script>

<WindowTitlebar
  controlsOrder="platform"
  windowControlsProps={{ platform: $isMac ? "macos" : "windows" }}
  class="h-9 w-full shrink-0 overflow-visible border-b bg-muted/40"
>
  <MenuBar />
  <div class="flex h-full flex-1 items-center gap-2 px-3" data-tauri-drag-region>
    <img src="/Buzee Logo.png" alt="Buzee" class="h-4 w-4" data-tauri-drag-region />
    <span class="text-sm font-semibold tracking-tight text-foreground" data-tauri-drag-region>
      Buzee
    </span>
  </div>
</WindowTitlebar>
