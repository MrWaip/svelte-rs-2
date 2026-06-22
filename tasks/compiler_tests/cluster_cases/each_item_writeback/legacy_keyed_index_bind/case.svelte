<svelte:options runes={false} />
<script>
	import Component from './Component.svelte';
	let items = [0];
</script>
{#each items as item, idx(item)}
	<Component bind:item />
{/each}
