<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { type ResolvedPathname } from '$app/types';

	import { isPathActive } from '$lib/utils';
	import {
		Compass,
		HardDrive,
		LayoutGrid,
		ScrollText,
		Settings2,
		type LucideIcon
	} from '@lucide/svelte';
</script>

<div class="bg-black/25 border-r border-surface-light flex flex-col">
	<ul class="flex flex-col gap-2 px-4 py-6 h-full border-b border-surface-light">
		{@render menu_item({
			label: 'Dashboard',
			href: resolve('/'),
			Icon: LayoutGrid
		})}

		{@render menu_item({
			label: 'Journeys',
			href: resolve('/journeys'),
			Icon: Compass
		})}

		{@render menu_item({
			label: 'Evidence',
			href: resolve('/evidence'),
			Icon: ScrollText
		})}

		{@render menu_item({
			label: 'Controller',
			href: resolve('/controller'),
			Icon: Settings2
		})}
	</ul>

	<div class="p-3">
		<div
			class="flex items-center gap-3 rounded border border-white/6
		       bg-white/[0.035] px-3 py-2.5"
		>
			<div class="grid size-10 shrink-0 place-items-center rounded-md bg-white/4 text-white/60">
				<HardDrive />
			</div>

			<div class="min-w-0 flex-1">
				<div class="truncate text-sm font-medium text-white/80">Google Drive</div>

				<div class="mt-0.5 flex items-center gap-1.5 text-white/55">
					<span class="size-1.5 rounded-full bg-emerald-400"></span>
					<span class="truncate text-xs opacity-85">Synced 2 min ago</span>
				</div>
			</div>
		</div>
	</div>
</div>

{#snippet menu_item({
	label,
	href,
	Icon
}: {
	label: string;
	href: ResolvedPathname;
	Icon: LucideIcon;
	icon_size?: number;
	is_active?: boolean;
})}
	<a
		{href}
		class="flex h-10 items-center gap-3 rounded-lg px-3 transition
		{isPathActive(page.url.pathname, href)
			? 'bg-linear-to-r from-emerald-200/4 to-emerald-400/16 text-emerald-200 border-2 border-emerald-500/75'
			: 'text-white/55 hover:bg-white/4 hover:text-white/75'}"
	>
		<Icon strokeWidth={isPathActive(page.url.pathname, href) ? 1.9 : 1.6} size={18} />
		<span class="truncate">{label}</span>
	</a>
{/snippet}

<style>
	@keyframes pulse {
		0%,
		100% {
			opacity: 1;
			transform: scale(1);
		}
		50% {
			opacity: 0.5;
			transform: scale(1.1);
		}
	}

	/* .pulse {
		animation: pulse 2.5s ease infinite;
	} */
</style>
