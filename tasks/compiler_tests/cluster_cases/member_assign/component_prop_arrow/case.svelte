<script>
	import Child from './Child.svelte';
	let ratings = $state([0, 1]);
</script>

{#each ratings as value, index}
	<Child onChange={(v) => (ratings[index] = v)} />
{/each}
