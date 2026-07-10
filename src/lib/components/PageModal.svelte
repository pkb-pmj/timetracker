<script lang="ts">
	import type { Attachment } from 'svelte/attachments';

	let { children } = $props();

	const pageDialogAttachment: Attachment<HTMLDialogElement> = (dialog) => {
		dialog.addEventListener('click', (e) => {
			if (e.target === dialog) dialog.close();
		});
		dialog.addEventListener('close', () => history.back());
		dialog.showModal();
	};
</script>

<dialog {@attach pageDialogAttachment}>
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
