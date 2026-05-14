<script>
	import { fade } from "svelte/transition";
	let tag = $state("div");
	let show = $state(true);
</script>

{#if show}
	<svelte:element this={tag} transition:fade|local>x</svelte:element>
{/if}
