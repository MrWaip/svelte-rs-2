<script>
	import { fade } from "svelte/transition";
	let tag = $state("div");
	let show = $state(true);
</script>

{#if show}
	<svelte:element this={tag} transition:fade|global>x</svelte:element>
{/if}
