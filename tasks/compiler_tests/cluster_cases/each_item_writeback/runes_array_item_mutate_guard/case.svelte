<svelte:options runes={true} />
<script>
	let cards = $state([{ fav: false }]);
</script>

{#each cards as card}
	<button onclick={() => { card.fav = !card.fav; }}>{card.fav}</button>
{/each}
