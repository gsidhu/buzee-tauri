<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { getVersion } from "@tauri-apps/api/app";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Button } from "$lib/components/ui/button/index.js";

  const win = getCurrentWindow();

  type MenuEntry =
    | { kind: "item"; label: string; shortcut?: string; action: () => void }
    | { kind: "separator" };

  type MenuDef = { label: string; entries: MenuEntry[] };

  function edit(cmd: string) {
    document.execCommand(cmd);
  }

  const menus: MenuDef[] = [
    {
      label: "File",
      entries: [
        { kind: "item", label: "Close Window", action: () => win.close() },
        { kind: "item", label: "Quit", action: () => win.destroy() },
      ],
    },
    {
      label: "Edit",
      entries: [
        { kind: "item", label: "Undo", shortcut: "Ctrl+Z", action: () => edit("undo") },
        { kind: "item", label: "Redo", shortcut: "Ctrl+Y", action: () => edit("redo") },
        { kind: "separator" },
        { kind: "item", label: "Cut", shortcut: "Ctrl+X", action: () => edit("cut") },
        { kind: "item", label: "Copy", shortcut: "Ctrl+C", action: () => edit("copy") },
        { kind: "item", label: "Paste", shortcut: "Ctrl+V", action: () => edit("paste") },
        { kind: "item", label: "Select All", shortcut: "Ctrl+A", action: () => edit("selectAll") },
      ],
    },
    {
      label: "Window",
      entries: [
        { kind: "item", label: "Minimize", action: () => win.minimize() },
        { kind: "item", label: "Maximize", action: () => win.toggleMaximize() },
        { kind: "separator" },
        { kind: "item", label: "Close Window", action: () => win.close() },
      ],
    },
    {
      label: "Help",
      entries: [{ kind: "item", label: "About Buzee", action: () => (aboutOpen = true) }],
    },
  ];

  let openMenu: string | null = null;
  let aboutOpen = false;
  let appVersion = "";
  getVersion()
    .then((v) => (appVersion = v))
    .catch(() => {});

  let menuBarEl: HTMLElement;

  function toggle(label: string) {
    openMenu = openMenu === label ? null : label;
  }

  function run(action: () => void) {
    openMenu = null;
    action();
  }

  function onMouseEnter(label: string) {
    if (openMenu) openMenu = label;
  }

  function onDocumentClick(e: MouseEvent) {
    if (!menuBarEl.contains(e.target as Node)) openMenu = null;
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") openMenu = null;
  }
</script>

<svelte:window on:click={onDocumentClick} on:keydown={onKeydown} />

<nav bind:this={menuBarEl} class="flex h-full shrink-0 items-stretch pl-1">
  {#each menus as menu}
    <div class="relative flex">
      <button
        type="button"
        class="flex h-full items-center px-2 text-[13px] font-medium leading-none text-foreground/80 transition-colors hover:bg-accent hover:text-accent-foreground"
        class:bg-accent={openMenu === menu.label}
        class:text-accent-foreground={openMenu === menu.label}
        on:click={() => toggle(menu.label)}
        on:mouseenter={() => onMouseEnter(menu.label)}
      >
        {menu.label}
      </button>
      {#if openMenu === menu.label}
        <div class="absolute left-0 top-full z-50 min-w-48 rounded-md border bg-popover p-1 text-popover-foreground shadow-md">
          {#each menu.entries as entry}
            {#if entry.kind === "separator"}
              <div class="-mx-1 my-1 h-px bg-muted"></div>
            {:else}
              <button
                type="button"
                class="flex w-full cursor-default select-none items-center justify-between gap-4 rounded-sm px-2 py-1.5 text-[13px] outline-none hover:bg-accent hover:text-accent-foreground"
                on:click={() => run(entry.action)}
              >
                <span>{entry.label}</span>
                {#if entry.shortcut}
                  <span class="ml-auto text-xs text-muted-foreground">{entry.shortcut}</span>
                {/if}
              </button>
            {/if}
          {/each}
        </div>
      {/if}
    </div>
  {/each}
</nav>

<Dialog.Root bind:open={aboutOpen}>
  <Dialog.Content class="max-w-sm">
    <Dialog.Header>
      <Dialog.Title class="flex items-center gap-3">
        <img src="/Buzee Logo.png" alt="Buzee" class="h-8 w-8" />
        <div>
          <div>Buzee</div>
          {#if appVersion}
            <div class="text-xs font-normal text-muted-foreground">Version {appVersion}</div>
          {/if}
        </div>
      </Dialog.Title>
      <Dialog.Description>Find your documents, effortlessly</Dialog.Description>
    </Dialog.Header>
    <Dialog.Footer>
      <Dialog.Close>
        <Button variant="outline">OK</Button>
      </Dialog.Close>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
