<script>
	let items = $state([{ done: false }]);
	let filtered = $derived(items);
</script>

{#each filtered as item}
	<input type="checkbox" bind:checked={item.done} />
{/each}
