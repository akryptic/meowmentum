<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { type ResolvedPathname } from '$app/types';

	console.log(page);

	import { isPathActive } from '$lib/utils';
	import { Compass, LayoutGrid, ScrollText, Settings2, type LucideIcon } from '@lucide/svelte';
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
			label: 'Evidece',
			href: resolve('/evidence'),
			Icon: ScrollText
		})}

		{@render menu_item({
			label: 'Controller',
			href: resolve('/controller'),
			Icon: Settings2
		})}
	</ul>

	<div class="p-4">

		<div class="p-4 border border-surface-light rounded-lg bg-surface-light/50">
			Todo: Sync Section
		</div>
	</div>
</div>

{#snippet menu_item({
	label,
	href,
	Icon,
	icon_size = 20
}: {
	label: string;
	href: ResolvedPathname;
	Icon: LucideIcon;
	icon_size?: number;
	is_active?: boolean;
})}
	<a
		{href}
		class="py-2 px-4 rounded-lg {isPathActive(page.url.pathname, href)
			? 'font-semibold bg-white/10 border-r-3 border-primary/75 bg-linear-to-r from-emeral-400y/0 to-primary/15 opacity-100 text-green-100'
			: 'hover:bg-linear-to-r from-white/5 to-primary/15 opacity-75'}"
	>
		<div class="flex text-lg items-center gap-2">
			<Icon strokeWidth={isPathActive(page.url.pathname, href) ? 2 : 1.5} size={icon_size} />
			{label}
		</div>
	</a>
{/snippet}
