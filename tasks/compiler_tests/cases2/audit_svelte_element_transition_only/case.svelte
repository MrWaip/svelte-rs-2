<script>
	import { fade } from "svelte/transition";
	let tag = $state("div");
</script>

<svelte:element this={tag} transition:fade>x</svelte:element>
