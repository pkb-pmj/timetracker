<script lang="ts">
	const resetDatabase = async () => {
		if (!window.confirm('Reset database? All data will be deleted')) return;
		const root = await navigator.storage.getDirectory();
		await root.removeEntry('database.sqlite3', { recursive: true });
		location.reload();
	};
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>
<a href="/nodes">Nodes</a>
<a href="/timeline">Timeline</a>
<button onclick={resetDatabase}>Reset database</button>
