<script lang="ts">
	import {
    Coffee,
		House,
		Search,
		Pin,
		PieChart,
		ScanText,
		Trophy,
		NotepadText,
		Lightbulb,
		Wind,
		Settings,
		ExternalLink,
		BusFront
	} from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button/index.js';
	import Separator from '$lib/components/ui/separator/separator.svelte';
	import { pagePath, isMac } from '$lib/stores';
	import { openInBrowser } from '$lib/utils/miscUtils';
</script>

<div class="flex flex-col h-full gap-2">
	<div class="flex items-center border-b px-4 h-[60px] min-h-[60px] lg:px-6">
		<a href="/" class="flex items-center gap-2 font-semibold">
			<!-- <Rocket class="h-6 w-6" /> -->
			<img id="buzee-logo-img" class="h-6 w-6" src="/Buzee Logo.png" alt="No Results" />
			<span class="">Buzee</span>
		</a>
		<Button href="/settings" variant="outline" size="icon" 
			class={`ml-auto h-8 w-8 ${$pagePath === '/settings' ? 'bg-[var(--very-light-purple)] text-primary' : 'text-muted-foreground'}`}
		>
			<Settings class="h-4 w-4" />
			<span class="sr-only">Settings</span>
		</Button>
	</div>
	<div class="flex-1">
		<nav class="grid items-start px-2 text-sm font-normal lg:px-4">
			<!-- <Label class="px-3 py-2 text-muted-foreground font-semibold">Locations</Label> -->
			<a
				href="/"
				class={`flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary ${$pagePath === '/' ? 'bg-[var(--very-light-purple)] text-primary' : 'text-muted-foreground'}`}
			>
				<House class="h-4 w-4" />
				Dashboard
			</a>
			<a
				href="/search"
				class={`flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary ${$pagePath === '/search' ? 'bg-[var(--very-light-purple)] text-primary' : 'text-muted-foreground'}`}
			>
				<Search class="h-4 w-4" />
				Search
			</a>
			<a
				href="##"
				class={`flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:cursor-not-allowed ${$pagePath === '/pinboard' ? 'bg-[var(--very-light-purple)] text-primary' : 'text-muted-foreground'}`}
			>
				<Pin class="h-4 w-4" />
				Pinned Items
				<!-- <Badge variant="outline" class="font-medium">Soon</Badge> -->
			</a>
			<!-- <a
				href="##"
				class={`flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary ${$pagePath === '/browser-history' ? 'bg-[var(--very-light-purple)] text-primary' : 'text-muted-foreground'}`}
			>
				<Compass class="h-4 w-4" />
				Browser History
			</a>
			<Accordion.Root class="px-3">
				<Accordion.Item class="border-0" value="item-1">
					<Accordion.Trigger class="hover:no-underline py-2">Coming Soon</Accordion.Trigger>
					<Accordion.Content>
						<a
							href="##"
							class={`flex items-center gap-3 rounded-lg py-2 transition-all hover:text-primary ${$pagePath === '/bookmarks' ? 'bg-[var(--very-light-purple)] text-primary' : 'text-muted-foreground'}`}
						>
							<Bookmark class="h-4 w-4" />
							Bookmarks
						</a>
						<a
							href="##"
							class={`flex items-center gap-3 rounded-lg py-2 transition-all hover:text-primary ${$pagePath === '/cloud' ? 'bg-[var(--very-light-purple)] text-primary' : 'text-muted-foreground'}`}
						>
							<Cloud class="h-4 w-4" />
							Cloud Storage
						</a>
					</Accordion.Content>
				</Accordion.Item>
			</Accordion.Root> -->
		</nav>
	</div>

	<Separator />

	<div class="flex-1">
		<nav class="grid items-start px-2 text-sm font-normal lg:px-4">
			<!-- <Label class="px-3 py-2 text-muted-foreground font-semibold">Magic Services</Label> -->
			<a
				href="/magic/extract-text"
				class={`flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary ${$pagePath === '/magic/extract-text' ? 'bg-[var(--very-light-purple)] text-primary' : 'text-muted-foreground'}`}
			>
				<ScanText class="h-4 w-4" />
				Extract Text
			</a>
			<a
				href="/magic/scratchpad"
				class={`flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary ${$pagePath === '/magic/scratchpad' ? 'bg-[var(--very-light-purple)] text-primary' : 'text-muted-foreground'}`}
			>
				<NotepadText class="h-4 w-4" />
				Scratchpad
			</a>
			<a
				href="##"
				on:click={() => openInBrowser('deadline')}
				class={`flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary ${$pagePath === '/magic/deadline' ? 'bg-[var(--very-light-purple)] text-primary' : 'text-muted-foreground'}`}
			>
				<Trophy class="h-4 w-4" />
				<span>
					Deadline
					<ExternalLink class="h-3 w-3 inline-block" />
				</span>
			</a>
			<a
				href="/magic/deep-breathing"
				class={`flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary ${$pagePath === '/magic/deep-breathing' ? 'bg-[var(--very-light-purple)] text-primary' : 'text-muted-foreground'}`}
			>
				<Wind class="h-4 w-4" />
				Deep Breathing
			</a>
		</nav>
	</div>

	<Separator />

	<div class="flex-1">
		<nav class="grid items-start px-2 text-sm font-normal lg:px-4">
			<a
				href="/magic/stats"
				class={`flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary ${$pagePath === '/magic/stats' ? 'bg-[var(--very-light-purple)] text-primary' : 'text-muted-foreground'}`}
			>
				<PieChart class="h-4 w-4" />
				Stats
			</a>
			<a
				href="/magic/tips"
				class={`flex items-center gap-3 rounded-lg px-3 py-2 transition-all hover:text-primary ${$pagePath === '/magic/tips' ? 'bg-[var(--very-light-purple)] text-primary' : 'text-muted-foreground'}`}
			>
				<Lightbulb class="h-4 w-4" />
				Tips & Shortcuts
			</a>
			<a
				href={"/gandalf?device" + (isMac ? "=mac" : "windows")}
				class={`flex items-center gap-3 rounded-lg px-3 py-2 transition-all ${$pagePath === '/gandalf' ? 'bg-[var(--very-light-purple)] text-primary' : 'text-muted-foreground'}`}
			>
				<BusFront class="h-4 w-4" />
				Roadmap Survey
				<!-- <Badge variant="outline" class="font-medium">Soon</Badge> -->
			</a>
		</nav>
	</div>

	<Separator />
  
	<div class="mt-auto flex flex-col p-4 text-center space-y-1">
    <p class="text-sm" id="app-version">Buzee v0.2.0</p>
    <Button id="bmac-btn" class="animate__headShake" variant="outlinePurple" on:click={() => openInBrowser('bmac')}>
      <Coffee id="bmac-btn-coffee-icon" class="mr-2 h-4 w-4" />
      Buy me a coffee
    </Button>
	</div>
</div>

<style>
	
</style>