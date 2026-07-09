<script lang="ts">
	import ModalShell from '$lib/components/layout/ModalShell.svelte';
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import Titlebar from '$lib/components/layout/Titlebar.svelte';
	import { type Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

<div class="w-full h-full grid grid-cols-[auto_1fr] grid-rows-[auto_1fr]">
	<Titlebar />
	<ModalShell />
	<Sidebar />
	{@render children()}
</div>
