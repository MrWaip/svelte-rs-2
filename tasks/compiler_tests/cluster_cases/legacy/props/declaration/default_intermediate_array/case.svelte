<svelte:options runes={false} />
<script>
	export let [[a, b] = [8, 9], c] = [[1, 2], 3];
</script>
<button>{a}{b}{c}</button>
