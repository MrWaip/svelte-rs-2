<svelte:options runes={false} />
<script>
	export let items;
</script>

{#each items as item}
	{#snippet row({ value })}
		<Child name={value.name} />
	{/snippet}
	{@render row({ value: item })}
{/each}
