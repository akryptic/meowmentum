<script lang="ts">
	import { AppWindow, Check, HardDrive } from '@lucide/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';

	let success = $state<'connected' | 'connecting' | 'failed'>('connecting');

	const bits = ['1', '0', '1', '1', '0', '0', '1', '0'];

	onMount(() => {
		setTimeout(async () => {
			alert(await invoke('close_drive_connect_window'));
		}, 6000);
	});
</script>

<div class="flex items-center justify-center gap-5 p-8 text-white">
	<div class="opacity-75 drop-shadow-lg drop-shadow-white/10">
		<HardDrive size={56} />
	</div>

	<div class="bit-stream flex justify-center items-center" aria-hidden="true">
		{#if success == 'connected'}
			<Check size={32} />
		{:else}
			{#each bits as bit, i (i + bit)}
				<span style={`animation-delay: ${i * 0.18}s`}>
					{bit}
				</span>
			{/each}
			<!-- <div class="pulse">
				<Link size={28} class="text-white"  />
			</div> -->
		{/if}
	</div>

	<div class="opacity-75">
		<AppWindow size={56} />
	</div>
</div>

<p class="text-center text-sm opacity-75 {success ? '' : 'pulse'}">
	{success == 'connected' ? 'Google Drive connected' : 'Connecting to Your Drive'}
</p>

<button onclick={() => (success = 'connected')}>Toggle</button>

<style>
	.bit-stream {
		position: relative;
		width: 150px;
		height: 42px;
		overflow: hidden;
	}

	.bit-stream::before {
		content: '';
		position: absolute;
		left: 0;
		right: 0;
		top: 50%;
		height: 2px;
		background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.25), transparent);
		transform: translateY(-50%);
	}

	.bit-stream span {
		position: absolute;
		left: -18px;
		top: 50%;
		font-size: 0.8rem;
		font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
		font-weight: 600;
		color: rgba(255, 255, 255, 0.85);
		text-shadow: 0 0 12px rgba(255, 255, 255, 0.45);
		animation: bit-flow 1.5s linear infinite;
	}

	.bit-stream span:nth-child(odd) {
		margin-top: -15px;
	}

	.bit-stream span:nth-child(even) {
		margin-top: 7px;
	}

	.pulse {
		animation: text-fade 1.5s ease-in-out infinite;
	}

	@keyframes bit-flow {
		0% {
			opacity: 0;
			transform: translateX(0) translateY(-50%) scale(0.8);
		}

		15% {
			opacity: 1;
		}

		80% {
			opacity: 1;
		}

		100% {
			opacity: 0;
			transform: translateX(170px) translateY(-50%) scale(1.05);
		}
	}

	@keyframes text-fade {
		0%,
		100% {
			opacity: 0.55;
		}

		50% {
			opacity: 1;
		}
	}
</style>
