<script lang="ts">
	import Titlebar from '$lib/components/layout/Titlebar.svelte';
	import { type Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

<div class="w-full h-full">
	<Titlebar />
	{@render children()}
</div>
