<svelte:options runes={false} />
<script>
	export let [a, , c] = [1, 2, 3];
</script>
<button>{a}{c}</button>
