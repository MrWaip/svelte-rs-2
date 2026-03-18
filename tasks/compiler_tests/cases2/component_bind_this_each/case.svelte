<script>
	import Component from './Component.svelte';
	let items = [1, 2, 3];
	let refs = $state([]);
</script>

{#each items as item, i}
	<Component bind:this={refs[i]} />
{/each}
