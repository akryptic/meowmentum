<script lang="ts">
	import { onMount } from 'svelte';
	import { Bell, Minus, Settings, UserRound, X, type LucideIcon } from '@lucide/svelte';
	import { invoke } from '@tauri-apps/api/core';
	let time = $state('');

	function updateTime() {
		time = new Date().toLocaleTimeString('en-US', {
			hour: 'numeric',
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
	class="col-span-2 border-b border-surface-light max-h-7 h-7 bg-black/25 flex items-center px-3 py-1 justify-between"
>
	<span class="inline-block leading-4 font-bold tracking-wider text-primary brightness-150">
		Meomentum
	</span>
	<span class="text-sm font-semibold text-white inline-block">{time}</span>
	<div class="text-sm font-extrabold flex gap-4 items-center text-white/50">
		<div class="flex pr-4 gap-3 border-r border-surface-light items-center">
			{@render icon_button({
				label: 'User',
				Icon: UserRound
			})}

			{@render icon_button({
				label: 'Notifications',
				Icon: Bell
			})}

			{@render icon_button({
				label: 'Settings',
				Icon: Settings
			})}
		</div>

		{@render icon_button({
			label: 'Minimize',
			Icon: Minus,
			onclick: () => invoke('minimize_application')
		})}

		{@render icon_button({
			label: 'Close',
			Icon: X,
			onclick: () => invoke('quit_application')
		})}
	</div>
</header>

{#snippet icon_button({
	label,
	Icon,
	icon_size = 16,
	stroke_width = 2,
	onclick
}: {
	label: string;
	Icon: LucideIcon;
	icon_size?: number;
	stroke_width?: number;
	onclick?: () => void;
})}
	<button title={label} {onclick} class="cursor-pointer active:scale-85 duration-150">
		<Icon strokeWidth={stroke_width} size={icon_size} />
	</button>
{/snippet}
