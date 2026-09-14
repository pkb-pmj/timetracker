<script lang="ts">
	import NodePicker from '$lib/components/NodePicker.svelte';
	import PageModal from '$lib/components/PageModal.svelte';
	import { invalidate } from '$lib/db';
	import { createActivity, createNode } from '$lib/db/queries';

	async function onPicked(id: number) {
		await createActivity(id);
		invalidate('db');
		history.back();
	}
</script>

<PageModal>
	<NodePicker {onPicked} {createNode} />
</PageModal>
