<script lang="ts">
  import { CircleUser } from "lucide-svelte";
  import * as DropdownMenu from "../ui/dropdown-menu";
  import { Button } from "$lib/components/ui/button/index.js";
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger asChild let:builder>
    <Button
      builders={[builder]}
      variant="secondary"
      size="icon"
      class="rounded-full"
    >
      <CircleUser class="h-5 w-5" />
      <span class="sr-only">Toggle user menu</span>
    </Button>
  </DropdownMenu.Trigger>
  <DropdownMenu.Content align="end">
    <DropdownMenu.Label>My Account</DropdownMenu.Label>
    <DropdownMenu.Separator />
    <DropdownMenu.Item>Settings</DropdownMenu.Item>
    <DropdownMenu.Item>Support</DropdownMenu.Item>
    <DropdownMenu.Separator />
    <DropdownMenu.Item>Logout</DropdownMenu.Item>
  </DropdownMenu.Content>
</DropdownMenu.Root>