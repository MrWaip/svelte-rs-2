<svelte:options runes={false} />
<script>
	export let field;
	let items = [{}];
</script>
{#each items as { [field]: value }}
	<button>{value}</button>
{/each}
