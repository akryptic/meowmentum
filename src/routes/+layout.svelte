<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { onMount, type Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();

	function disableMenu() {
		if (window.location.hostname !== 'tauri.localhost') {
			return;
		}

		document.addEventListener(
			'contextmenu',
			(e) => {
				e.preventDefault();
				return false;
			},
			{ capture: true }
		);

		document.addEventListener(
			'selectstart',
			(e) => {
				e.preventDefault();
				return false;
			},
			{ capture: true }
		);
	}

	onMount(() => {
		disableMenu();
	});
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
{@render children()}
