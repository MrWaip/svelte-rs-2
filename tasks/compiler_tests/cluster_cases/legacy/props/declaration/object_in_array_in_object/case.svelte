<svelte:options runes={false} />
<script>
	export let { outer: [{ inner }] } = { outer: [{ inner: 1 }] };
</script>
<button>{inner}</button>
