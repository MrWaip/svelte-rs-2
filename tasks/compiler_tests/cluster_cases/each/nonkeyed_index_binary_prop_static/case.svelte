<svelte:options runes={false} />

<script>
	import Child from './Child.svelte';
	export let list;
</script>

{#each list || [] as item, idx}
	<Child label="ID ({idx + 1})" />
{/each}
