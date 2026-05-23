<script>
	import { fade, fly } from "svelte/transition";
	let tag = $state("div");
	let show = $state(true);
	let duration = $state(300);
</script>

{#if show}
	<svelte:element this={tag} in:fade={{ duration }} out:fly={{ duration }}>x</svelte:element>
{/if}
