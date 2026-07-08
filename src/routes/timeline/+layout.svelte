<script lang="ts">
	import Sequence from '$lib/components/Sequence.svelte';

	let { children } = $props();
</script>

<a href="/">Home</a>
<Sequence />

{@render children()}
