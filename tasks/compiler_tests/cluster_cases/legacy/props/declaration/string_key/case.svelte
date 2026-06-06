<svelte:options runes={false} />
<script>
	export let { "a-b": ab, "c d": cd } = { "a-b": 1, "c d": 2 };
</script>
<button>{ab}{cd}</button>
