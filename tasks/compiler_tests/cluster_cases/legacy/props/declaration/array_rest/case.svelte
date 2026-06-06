<svelte:options runes={false} />
<script>
	export let [a, ...rest] = [1, 2, 3];
</script>
<button>{a}{rest.length}</button>
