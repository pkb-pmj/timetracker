<script lang="ts">
	import { browser } from '$app/environment';
	import '$lib/style.css';

	let { children } = $props();

	if (browser) {
		console.log("waiting for 'beforeinstallprompt'");
	}

	function onBeforeInstallPrompt(e: Event) {
		// Prevent the mini-infobar from appearing on mobile
		// e.preventDefault();
		// Stash the event so it can be triggered later.
		// deferredPrompt = e;
		// Update UI notify the user they can install the PWA
		// showInstallPromotion();
		// Optionally, send analytics event that PWA install promo was shown.
		console.log(`'beforeinstallprompt' event was fired.`);
		alert(`'beforeinstallprompt' event was fired.`)
	}
</script>

<svelte:window onbeforeinstallprompt={onBeforeInstallPrompt} />

{@render children()}
