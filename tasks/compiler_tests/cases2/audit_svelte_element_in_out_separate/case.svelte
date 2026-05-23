<script>
	import { fade, fly } from "svelte/transition";
	let tag = $state("div");
	let show = $state(true);
</script>

{#if show}
	<svelte:element this={tag} in:fade out:fly>x</svelte:element>
{/if}
