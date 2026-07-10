import type { LucideIcon } from '@lucide/svelte';
import type { Component, Snippet } from 'svelte';

export interface ModalProps {
	title: string;
	icon?: LucideIcon;
	body?: Component;

	quickActions?: Snippet;

	isOpen: boolean;
	close: () => void;
	toggle: () => void;
	open: (
		props: Omit<ModalProps, 'show' | 'isOpen' | 'close' | 'toggle' | 'open' | 'quickActions'>
	) => void;
}

class Modal implements ModalProps {
	#show = $state(false);
	#quickActions = $state<Snippet | undefined>();

	title = $state('');
	icon = $state<LucideIcon | undefined>();
	body = $state<Component | undefined>();

	open(props: Omit<ModalProps, 'show' | 'isOpen' | 'close' | 'toggle' | 'open' | 'quickActions'>) {
		this.#show = true;
		this.title = props.title;
		this.icon = props.icon;
		this.body = props.body;
	}

	close() {
		this.#show = false;
		this.title = '';
		this.icon = undefined;
		this.body = undefined;
	}

	toggle() {
		this.#show = !this.#show;
	}

	setQuickActions(actions: Snippet) {
		this.#quickActions = actions;
	}

	get isOpen() {
		return this.#show;
	}

	get quickActions() {
		return this.#quickActions;
	}
}

/**
 * Singleton instance used throughout the application.
 */
const modalManager = new Modal();

export default modalManager;
