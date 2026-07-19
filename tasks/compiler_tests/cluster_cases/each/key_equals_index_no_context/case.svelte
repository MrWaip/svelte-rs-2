<script>
	let items = $state([1, 2, 3]);
</script>

{#each items, idx (idx)}
	<button onclick={() => items[idx]++}>{idx}</button>
{/each}
