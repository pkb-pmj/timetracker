<script lang="ts">
	import NodePicker from '$lib/components/NodePicker.svelte';
	import { db, invalidate } from '$lib/db';
	import type { Attachment } from 'svelte/attachments';

	async function createNode(name: string) {
		return (await db.insertInto('nodes').values({ name }).returning('id').executeTakeFirstOrThrow())
			.id;
	}

	async function createEvent(node_id: number) {
		await db
			.insertInto('events')
			.values({
				time: Date.now(),
				node_id,
			})
			.returning('id')
			.executeTakeFirstOrThrow();
		invalidate('db');
	}

	async function onPicked(id: number) {
		await createEvent(id);
		history.back();
	}

	const pageDialogAttachment: Attachment<HTMLDialogElement> = (dialog) => {
		dialog.addEventListener('click', (e) => {
			if (e.target === dialog) dialog.close();
		});
		dialog.addEventListener('close', () => history.back());
		dialog.showModal();
	};
</script>

<dialog {@attach pageDialogAttachment}>
	<NodePicker {onPicked} {createNode} />
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
