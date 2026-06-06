<svelte:options runes={false} />
<script>
	export let { p: { a } = {} } = {};
</script>
<button>{a}</button>
