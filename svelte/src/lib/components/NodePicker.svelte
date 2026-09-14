<script lang="ts">
	import { db, type Node } from '$lib/db';
	import type { Selectable } from 'kysely';
	import type { Attachment } from 'svelte/attachments';

	let {
		onPicked,
		createNode,
	}: { onPicked: (id: number) => void; createNode: (name: string) => Promise<number> } = $props();

	let query = $state('');
	let debouncedQuery = $state('');

	$effect(() => {
		const q = query;
		const t = setTimeout(() => {
			debouncedQuery = q;
		}, 200);
		return () => clearTimeout(t);
	});

	let nodes = $derived(
		await db
			.selectFrom('nodes')
			.selectAll()
			.where('name', 'like', `%${debouncedQuery}%`)
			.limit(30)
			.execute(),
	);

	let activeId = $state<number | null>(null);
	let selected = $state<Selectable<Node> | null>(null);

	function scrollIntoView(id: number): Attachment {
		// make sure active node is visible
		return (el) => {
			if (activeId === id) el.scrollIntoView({ block: 'nearest' });
		};
	}

	$effect(() => {
		// reset activeId if it was filtered out
		if (!nodes.some((node) => node.id === activeId)) activeId = null;
	});

	let inputEl = $state<HTMLInputElement | null>(null);

	function moveActiveOption(step: number) {
		if (activeId === null) {
			activeId = nodes[0].id;
		} else {
			const i = nodes.findIndex((node) => node.id === activeId);
			activeId = nodes[(i + step + nodes.length) % nodes.length].id;
		}
	}

	function selectOption(id: number) {
		activeId = id;
		selected = nodes.find((node) => node.id === id) ?? null;
		onPicked(id);
	}

	function onKeyDown(e: KeyboardEvent) {
		if (e.key === 'ArrowUp') {
			e.preventDefault();
			moveActiveOption(-1);
		} else if (e.key === 'ArrowDown') {
			e.preventDefault();
			moveActiveOption(1);
		} else if (e.key === 'Enter') {
			e.preventDefault();
			if (activeId !== null) selectOption(activeId);
		}
	}

	function onClose() {
		activeId = null;
		query = '';
	}

	async function onCreateNew() {
		onPicked(await createNode(query));
	}
</script>

<div class="combobox">
	<!-- svelte-ignore a11y_autofocus -->
	<input
		type="text"
		role="combobox"
		aria-autocomplete="list"
		aria-expanded="true"
		aria-controls="listbox"
		aria-activedescendant={activeId !== null ? `option-${activeId}` : undefined}
		autocomplete="off"
		placeholder="Search"
		autocapitalize="words"
		autofocus
		onkeydown={onKeyDown}
		bind:value={query}
		bind:this={inputEl}
	/>
	<ul id="listbox" role="listbox" tabindex="-1">
		{#each nodes as node (node.id)}
			<li
				id="option-{node.id}"
				role="option"
				aria-selected={activeId === node.id}
				onmousedown={(e) => {
					e.preventDefault();
					selectOption(node.id);
				}}
				{@attach scrollIntoView(node.id)}
			>
				<span class="name">{node.name}</span> <span class="id">#{node.id}</span>
			</li>
		{:else}
			<span class="no-elements">No matching options</span>
			<button class="create" onclick={onCreateNew}>Create new node: {query}</button>
		{/each}
	</ul>
</div>

<style>
	.combobox {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		padding: 0.5rem;
		flex: 1;
		min-height: 0;
	}

	input {
		background: white;
		padding: 0.5rem;
		border-radius: 0.5rem;
		border: 1px solid #ccc;
		font-size: 1rem;
	}

	ul {
		flex: 1;
		overflow-y: auto;
		padding: 0;
		margin: 0;
		list-style: none;
		border-radius: 0.5rem;
	}

	li {
		padding: 0.5rem;
		cursor: pointer;
		border-radius: 0.5rem;
	}

	/* Mouse hover */
	li:hover {
		background: rgba(0, 0, 0, 0.05);
	}

	/* Active (== selected) (arrow-key focus) */
	li[aria-selected='true'] {
		background: rgba(0, 0, 0, 0.12);
	}

	span.id {
		font-size: 0.8rem;
		color: #888;
	}

	span.no-elements {
		color: #888;
		font-style: italic;
		display: block;
		padding: 0.5rem 1rem;
	}

	button.create {
		background: white;
		padding: 0.5rem 1rem;
		border-radius: 0.5rem;
		border: 1px solid green;
		font-size: 1rem;
		width: 100%;
		text-align: left;
	}
</style>
