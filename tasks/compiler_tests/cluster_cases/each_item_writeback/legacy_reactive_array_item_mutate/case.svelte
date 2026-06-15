<svelte:options runes={false} />
<script>
	let cards = [{ fav: false }];
</script>

{#each cards as card}
	<button on:click={() => { card.fav = !card.fav; }}>{card.fav}</button>
{/each}
