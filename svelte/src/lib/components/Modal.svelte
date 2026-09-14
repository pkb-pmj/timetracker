<script lang="ts">
	import { pushState } from '$app/navigation';
	import { page } from '$app/state';

	let { id, children, onClose }: { id: string; children: any; onClose?: () => void } = $props();

	// used only inside user-triggered functions and $effect, which runs after mount, so should be safe without null
	let dialogEl: HTMLDialogElement;
	let isOpen = $state(false);

	function onClickOutside(e: MouseEvent) {
		if (e.target === dialogEl) dialogEl.close();
	}

	// TODO: a less hacky way to do this..? And be careful when integrating this with other modals
	export function open() {
		// only push state if it isn't already set to `id`
		// when user opens model, closes, navigates forward and opens modal again,
		// this prevents duplicate entries and allows closing modal on 1st "back", not 2nd
		if (page.state.modal !== id) pushState('', { modal: id });
		isOpen = true;
		dialogEl.showModal();
	}

	export function close() {
		dialogEl.close();
	}

	$effect(() => {
		// only close on "back", don't open on "forward"
		if (page.state.modal !== id) dialogEl.close();
	});

	function onclose() {
		// update history if closed in other way than navigating back
		if (page.state.modal === id) history.back();
		if (onClose) onClose();
		isOpen = false;
	}
</script>

<dialog bind:this={dialogEl} onclick={onClickOutside} {onclose}>
	{@render children()}
</dialog>

<style>
	dialog {
		padding: 0;
		border: none;
		border-radius: 1rem;
		width: 80%;
		max-width: 600px;
		margin: 4rem auto;
		transform: translateY(100%);
		transition: transform 150ms ease-out;
		max-height: calc(100dvh - 4rem);
		flex-direction: column;
	}

	dialog[open] {
		transform: translateY(0);
		display: flex;
	}

	dialog::backdrop {
		background: rgba(0, 0, 0, 0.3);
	}
</style>
