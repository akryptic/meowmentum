<script>
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	let counter = $state(0);

	onMount(() => {
		const inter = setInterval(() => counter++, 1000);

		return () => clearInterval(inter);
	});
</script>

<button
	onclick={() => {
		invoke('launch_drive_connect_window');
	}}
>
	Launch Connector Window
</button>