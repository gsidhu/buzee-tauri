<script lang="ts">
	import { Command as CommandPrimitive } from "cmdk-sv";
	import { cn } from "$lib/utils.js";
	import { Search, CalendarIcon } from "lucide-svelte";

	type $$Props = CommandPrimitive.InputProps;

	let className: string | undefined | null = undefined;
	export { className as class };
	export let value: string = "";
</script>

<div class="flex items-center px-2 border-b" data-cmdk-input-wrapper="">
	{#if className?.indexOf("calendar") === -1}
		<Search class="mr-2 h-4 w-4 shrink-0 opacity-50" />
	{:else}
		<CalendarIcon class="mr-2 h-4 w-4 shrink-0 opacity-50" />
	{/if}
	<CommandPrimitive.Input
		class={cn(
			"flex h-11 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50",
			className
		)}
		{...$$restProps}
		bind:value
	/>
</div>
