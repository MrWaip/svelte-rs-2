<svelte:options runes={false} />
<script>
	function make() { return 1; }
	export let x = make();
</script>
<p>{x}</p>
