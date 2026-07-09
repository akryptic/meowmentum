import type { LucideIcon } from '@lucide/svelte';
import type { Component } from 'svelte';

export interface ModalProps {
	title: string;
	icon?: LucideIcon;
	body?: Component;

	isOpen: boolean;
	close: () => void;
	toggle: () => void;
	open: (props: Omit<ModalProps, 'show' | 'isOpen' | 'close' | 'toggle' | 'open'>) => void;
}

class Modal implements ModalProps {
	#show = $state(false);
	title = $state('');
	icon = $state<LucideIcon | undefined>();
	body = $state<Component | undefined>();

	open(props: Omit<ModalProps, 'show' | 'isOpen' | 'close' | 'toggle' | 'open'>) {
		// if (this.#show) this.close();

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

	get isOpen() {
		return this.#show;
	}
}

/**
 * Singleton instance used throughout the application.
 */
const modalManager = new Modal();

export default modalManager;
