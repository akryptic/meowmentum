<script lang="ts">
	import type { LucideIcon } from "@lucide/svelte";

	interface Props {
		label: string;
		Icon: LucideIcon;
		icon_size?: number;
		stroke_width?: number;
		onclick?: () => void;
	}

	const { label, Icon, icon_size = 15, stroke_width = 1.8, onclick }: Props = $props();
</script>

<button
	title={label}
	{onclick}
	class="grid size-5 place-items-center rounded text-white/45
		       transition duration-150 hover:bg-white/6 cursor-pointer hover:text-white/80
		       active:scale-90"
>
	<Icon strokeWidth={stroke_width} size={icon_size} />
</button>
