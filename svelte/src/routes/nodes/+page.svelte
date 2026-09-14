<script lang="ts">
	import NodePicker from '$lib/components/NodePicker.svelte';
	import { db, reactiveQuery } from '$lib/db';

	const nodes = reactiveQuery(db.selectFrom('nodes').selectAll().orderBy('id', 'asc').compile());

	let newName = $state('');

	let newNameValid = $derived(
		newName.trim() && !$nodes.find((node) => node.name === newName.trim()),
	);

	const addNode = async () => {
		if (!newNameValid) return;
		await db.insertInto('nodes').values({ name: newName.trim() }).execute();
		newName = '';
	};

	const deleteNode = async (id: number) => {
		await db.deleteFrom('nodes').where('id', '=', id).execute();
	};
</script>

<a href="/">Home</a>
<ul>
	{#each $nodes as node}
		<li>
			<span>{node.name}</span>
			<span>#{node.id}</span>
			<button onclick={() => deleteNode(node.id)}>Delete</button>
		</li>
	{/each}
</ul>
<form>
	<input type="text" autocapitalize="words" bind:value={newName} />
	<button type="submit" onclick={addNode} disabled={!newNameValid}>Add node</button>
</form>
<NodePicker onPicked={() => {}} />
