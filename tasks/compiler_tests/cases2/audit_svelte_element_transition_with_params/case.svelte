<script>
	import { fade } from "svelte/transition";
	let tag = $state("div");
	let duration = $state(300);
</script>

<svelte:element this={tag} transition:fade={{ duration }}>x</svelte:element>
