<script lang="ts">
  import * as Select from "$lib/components/ui/select";
	import { Label } from "$lib/components/ui/label/index.js";
	import { resultsPerPage } from '$lib/stores';

	let selectedLocationOption = { value: $resultsPerPage, label: $resultsPerPage.toString() };
</script>

<div class="flex flex-col w-full">
	<Label class="mb-2 font-medium">Results Per Page</Label>
	<Select.Root bind:selected={selectedLocationOption} onSelectedChange={(v) => v?.value ? $resultsPerPage = v.value : '' }>
		<Select.Trigger class="">
			<Select.Value placeholder="25" />
		</Select.Trigger>
		<Select.Content>
				<Select.Item value="25">10</Select.Item>
				<Select.Item value="25">25</Select.Item>
				<Select.Item value="25">50</Select.Item>
				<Select.Item value="25">100</Select.Item>
		</Select.Content>
	</Select.Root>
</div>