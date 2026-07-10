<script lang="ts">
	import NodePicker from '$lib/components/NodePicker.svelte';
	import { db, depends, invalidate } from '$lib/db';
	import { createActivity, createNode, getItems } from '$lib/db/queries';
	import Modal from './Modal.svelte';
	import Timeline from './Timeline.svelte';

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
		invalidate('db');
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
