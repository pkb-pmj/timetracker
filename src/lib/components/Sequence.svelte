<script lang="ts">
	import NodePicker from '$lib/components/NodePicker.svelte';
	import { db, depends, invalidate } from '$lib/db';
	import { getItems } from '$lib/db/queries';
	import Modal from './Modal.svelte';
	import Timeline from './Timeline.svelte';

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

	async function createActivity(node_id: number) {
		await db
			.insertInto('activities')
			.values({
				node_id,
				start_time: Date.now(),
				end_time: null,
			})
			.executeTakeFirstOrThrow();
		invalidate('db');
	}

	async function finishAllActivities() {
		await db
			.updateTable('activities')
			.set({ end_time: Date.now() })
			.where('end_time', 'is', null)
			.execute();
		invalidate('db');
	}

	let activityModal: Modal;

	async function onActivityPicked(id: number) {
		await createActivity(id);
		activityModal.close();
	}

	let { events, activities, intervals } = $derived(await depends('db', getItems()));
</script>

<div class="container">
	<Timeline {events} {activities} {intervals} />
	<div class="buttons-container">
		<button onclick={() => activityModal.open()}>New Activity</button>
		<button onclick={finishAllActivities}>Finish All Activities</button>
		<a href="/timeline/events/new">New Event</a>
	</div>
</div>
<Modal id="/timeline/activities/new" bind:this={activityModal}>
	<NodePicker onPicked={onActivityPicked} {createNode} />
</Modal>

<style>
	.container {
		display: flex;
		flex-direction: column;
		padding: 0.5rem;
		gap: 0.5rem;
		border-radius: 1rem;
		border: 1px solid #ccc;
	}
	.buttons-container {
		display: flex;
		flex-direction: row;
		gap: 0.5rem;
		justify-content: stretch;
	}
	button,
	a {
		flex: 1;
		min-width: fit-content;
		background: white;
		padding: 0.25rem 0.75rem;
		border-radius: 0.5rem;
		border: 1px solid green;
		font-size: 1rem;
		cursor: pointer;
		text-align: center;
	}
</style>
