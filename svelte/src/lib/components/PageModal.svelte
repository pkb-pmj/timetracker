<script lang="ts">
	import { tick } from 'svelte';
	import type { Attachment } from 'svelte/attachments';

	let { children } = $props();

	const pageDialogAttachment: Attachment<HTMLDialogElement> = (dialog) => {
		dialog.addEventListener('click', (e) => {
			if (e.target === dialog) dialog.close();
		});
		dialog.addEventListener('close', () => history.back());
		dialog.showModal();
		tick().then(() => (hidden = false));
	};

	let hidden = $state(true);
</script>

<dialog {@attach pageDialogAttachment} class:hidden>
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
		transform: translateY(0);
		transition: transform 100ms ease-out;
		max-height: calc(100dvh - 4rem);
		flex-direction: column;
		&.hidden {
			transform: translateY(100%);
		}
		&[open] {
			display: flex;
		}
		&::backdrop {
			background: rgba(0, 0, 0, 0.3);
		}
	}
</style>
