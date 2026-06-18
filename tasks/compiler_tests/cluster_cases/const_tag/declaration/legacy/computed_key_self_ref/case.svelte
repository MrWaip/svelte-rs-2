<svelte:options runes={false} />
<script>
	let boxes = [{ k1: 'a' }];
	let area = '';
</script>
{#each boxes as box}
	{@const { i = 1, [`k${i}`]: sideone, [`k${area}${i + 1}`]: sidetwo } = box}
	<button>{sideone}{sidetwo}{i}</button>
{/each}
