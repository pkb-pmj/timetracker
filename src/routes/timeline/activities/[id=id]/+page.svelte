<script lang="ts">
	import PageModal from '$lib/components/PageModal.svelte';
	import { db, depends, invalidate } from '$lib/db';
	import { formatTime } from '$lib/util';
	import type { PageProps } from './$types';

	let { params }: PageProps = $props();

	let data = $derived(
		await depends(
			'db',
			db
				.selectFrom('activities')
				.innerJoin('nodes', 'nodes.id', 'activities.node_id')
				.select([
					'activities.id as id',
					'activities.start_time as startTime',
					'activities.end_time as endTime',
					'activities.node_id as nodeId',
					'nodes.name as nodeName',
				])
				.where('activities.id', '=', parseInt(params.id))
				.executeTakeFirstOrThrow(),
		),
	);

	async function finishActivity(id: number) {
		await db
			.updateTable('activities')
			.set({ end_time: Date.now() })
			.where('id', '=', id)
			.returning('id')
			.executeTakeFirstOrThrow();
		invalidate('db');
	}
</script>

<PageModal>
	<div class="container">
		#{data.id}
		{data.nodeName}
		{formatTime(data.startTime)} – {data.endTime ? formatTime(data.endTime) : '?'}
		{#if data.endTime === null}
			<button onclick={() => finishActivity(data.id)}>Finish Activity</button>
		{/if}
		<input type="time">
	</div>
</PageModal>

<style>
	.container {
		padding: 0.5em;
	}
	button {
		border: 1px solid darkgreen;
		border-radius: 0.5em;
		padding: 0.25em 0.5em;
		cursor: pointer;
	}
</style>
