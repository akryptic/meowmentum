<script lang="ts">
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import Titlebar from '$lib/components/layout/Titlebar.svelte';
	import { type Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

<div class="w-full h-full grid grid-cols-[16rem_1fr] grid-rows-[auto_1fr]">
	<Titlebar />
	<Sidebar />
	{@render children()}
</div>
