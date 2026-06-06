<svelte:options runes={false} />
<script>
	export let [a, ...[b, c]] = [1, 2, 3];
</script>
<button>{a}{b}{c}</button>
