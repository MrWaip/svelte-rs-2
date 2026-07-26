<svelte:options runes={false} />
<script>
	import Child from './Child.svelte';
	export let rows;
</script>

{#each rows as [first]}
	<Child bind:value={first} />
{/each}
