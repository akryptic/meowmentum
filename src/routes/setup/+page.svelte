<script lang="ts">
	import Loader from '$lib/components/ui/Loader.svelte';
	import { delay } from '$lib/utils';
	import { onDestroy, onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	type SetupError = {
		code: string;
		message: string;
		recoverable: boolean;
		context?: Record<string, unknown>;
	};

	let state = $state({
		isLoading: true,
		loadingText: 'Getting things ready',
		loadingDots: 0,
		errorText: '',
		errorJson: null as SetupError | null,
		windowWidth: 0,
		windowHeight: 0
	});

	let loadingInterval: ReturnType<typeof setInterval> | undefined;

	function parseSetupError(error: unknown) {
		const text = error instanceof Error ? error.message : String(error);

		try {
			state.errorJson = JSON.parse(text);
			state.errorText = '';
		} catch {
			state.errorJson = null;
			state.errorText = text;
		}
	}

	function startLoadingText() {
		stopLoadingText();

		loadingInterval = setInterval(() => {
			if (state.loadingDots === 3) {
				state.loadingDots = 0;
				state.loadingText = 'Hold on a little';
			} else {
				state.loadingText += ' .';
				state.loadingDots++;
			}
		}, 500);
	}

	function stopLoadingText() {
		if (loadingInterval) {
			clearInterval(loadingInterval);
			loadingInterval = undefined;
		}
	}

	async function setup() {
		state.windowWidth = Math.round(window.screen.width * 0.6);
		state.windowHeight = Math.round(window.screen.height * 0.6);

		await invoke('create_main_window', {
			windowWidth: state.windowWidth,
			windowHeight: state.windowHeight
		});

		await delay(3000);

		await invoke('launch_main_window');
	}

	async function boot() {
		state.isLoading = true;
		state.errorText = '';
		state.errorJson = null;

		startLoadingText();

		try {
			await setup();
		} catch (error) {
			stopLoadingText();
			state.loadingText = '';
			parseSetupError(error);
		} finally {
			state.isLoading = false;
		}
	}

	onMount(() => {
		boot();
	});

	onDestroy(() => {
		stopLoadingText();
	});
</script>

<main data-tauri-drag-region class="flex h-full w-full flex-col items-center justify-center">
	<Loader --color="" --scale="1.75" --speed="1" />

	{#if state.loadingText}
		<h1 class="fixed bottom-16 font-semibold tracking-widest opacity-75">
			{state.loadingText}
		</h1>
	{/if}

	{#if state.errorJson}
		<pre
			class="absolute bottom-8 max-w-[90%] overflow-auto rounded-lg bg-red-950/30 p-4 text-sm text-red-200/90">
{JSON.stringify(state.errorJson, null, 2)}
		</pre>
	{:else if state.errorText}
		<p class="absolute bottom-8 max-w-[90%] text-center text-xl text-red-300/75">
			{state.errorText}
		</p>
	{/if}
</main>
