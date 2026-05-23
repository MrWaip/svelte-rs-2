<svelte:options runes={false} />
<script>
	let w = 0;
</script>

<svelte:window bind:innerWidth={w} />
<div>{w}</div>
