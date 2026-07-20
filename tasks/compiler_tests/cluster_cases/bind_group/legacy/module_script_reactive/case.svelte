<svelte:options runes={false} />
<script module>
	export const meta = { title: "x" };
</script>
<script>
	let one = 1;
	let doubled = 0;
	$: doubled = one * 2;
</script>

<input type="radio" bind:group={one} value={1} />
<input type="radio" bind:group={one} value={2} />
<span>{doubled}</span>
