<svelte:options runes={false} />
<script>
	let p = Promise.resolve([{ fav: false }]);
</script>

{#await p then cards}
	{#each cards.filter((c) => !c.fav) as card}
		<button on:click={() => { card.fav = !card.fav; }}>{card.fav}</button>
	{/each}
{/await}
