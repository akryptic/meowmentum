<script lang="ts">
	import modalManager from '$lib/store/modal.svelte';
	import { Bell, Minus, Settings, UserRound, X } from '@lucide/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import IconButton from '$lib/components/ui/IconButton.svelte';
	import MSettings from '$lib/components/modal/settings/Modal.svelte';
	let time = $state('');

	function updateTime() {
		time = new Date().toLocaleTimeString('en-GB', {
			hour: '2-digit',
			minute: '2-digit',
			second: '2-digit'
		});
	}

	onMount(() => {
		updateTime();

		const inter = setInterval(updateTime, 1000);

		return () => clearInterval(inter);
	});
</script>

<header
	data-tauri-drag-region
	class="col-span-2 flex h-7 max-h-7 items-center justify-between relative
	       border-b border-surface-light bg-black/25 px-3 z-999999999"
>
	<span class="text-sm leading-5 font-semibold tracking-[0.02em] text-emerald-400">
		Meowmentum
	</span>

	<span
		class="text-xs leading-5 font-medium tabular-nums text-white/80 absolute left-1/2 top-1/2 -translate-1/2"
	>
		{time}
	</span>

	<div class="flex items-center gap-3 text-white/45">
		<div class="flex items-center gap-2.5 border-r border-surface-light pr-3">
			<IconButton
				label="User"
				Icon={UserRound}
				onclick={() =>
					modalManager.open({
						title: 'Profile',
						icon: UserRound
					})}
			/>
			<IconButton label="Notifications" Icon={Bell} />
			<IconButton
				label="Settings"
				Icon={Settings}
				onclick={() =>
					modalManager.open({
						title: 'Settings',
						icon: Settings,
						body: MSettings
					})}
			/>
		</div>

		<IconButton label="Minimize" Icon={Minus} onclick={() => invoke('minimize_application')} />
		<IconButton label="Close" Icon={X} onclick={() => invoke('quit_application')} />
	</div>
</header>
