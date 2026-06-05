<svelte:options runes={false} />
<script>
	export let [a = 10, b = 20] = [1];
</script>
<button>{a}{b}</button>
