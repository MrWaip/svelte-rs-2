<svelte:options runes={false} />
<script>
	export let [{ a }, { b }] = [{ a: 1 }, { b: 2 }];
</script>
<button>{a}{b}</button>
