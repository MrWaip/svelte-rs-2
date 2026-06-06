<svelte:options runes={false} />
<script>
	export let [[a, b], [c, d]] = [[1, 2], [3, 4]];
</script>
<button>{a}{b}{c}{d}</button>
