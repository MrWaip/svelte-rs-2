<svelte:options runes={false} />
<script>
	export let [a, b] = [1, 2];
</script>
<button>{a}{b}</button>
