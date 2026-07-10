<script lang="ts">
	import modalManager from '$lib/store/modal.svelte';
	import { X } from '@lucide/svelte';
	import IconButton from '../ui/IconButton.svelte';
	import { blur } from 'svelte/transition';
	import { onMount } from 'svelte';

	function handleKeyPress(e: KeyboardEvent) {
		if (e.key === 'Escape' && modalManager.isOpen) {
			modalManager.close();
		}
	}

	onMount(() => {
		window.addEventListener('keydown', handleKeyPress);

		return () => {
			window.removeEventListener('keydown', handleKeyPress);
		};
	});
</script>

{#if modalManager.isOpen}
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		transition:blur
		class="fixed bg-black/50 h-full w-full left-0 top-0 flex items-center justify-center backdrop-blur-xs z-999"
		onclick={(e) => {
			if (e.target === e.currentTarget) {
				modalManager.close();
			}
		}}
	>
		<div class="border-2 border-surface-light w-[80%] h-[80%] bg-surface z-9999 rounded-md">
			<div
				class="bg-black/25 py-1 px-3 text-sm border-b border-surface-light flex items-center gap-2 text-white/80"
			>
				<modalManager.icon size={14} strokeWidth={1.5} class="opacity-75" />
				{modalManager.title}

				<div class="ml-auto flex items-center gap-3 text-white/45">
					{#if modalManager.quickActions}
						<div class="flex items-center gap-2.5 border-r border-surface-light pr-3">
							{@render modalManager.quickActions()}
						</div>
					{/if}
					<IconButton label="Close" Icon={X} onclick={() => modalManager.close()} />
				</div>
			</div>
			<modalManager.body />
		</div>
	</div>
{/if}
