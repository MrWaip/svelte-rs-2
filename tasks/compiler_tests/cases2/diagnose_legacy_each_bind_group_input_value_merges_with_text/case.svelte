<svelte:options runes={false} />
<script>
	export let items;
	export let value;
</script>

{#each items as item (item.value)}
	<label>
		<span>{item.label}</span>
		<input type="radio" value={item.value} bind:group={value} />
	</label>
{/each}
