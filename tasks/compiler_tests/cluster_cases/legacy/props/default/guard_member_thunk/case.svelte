<svelte:options runes={false} />
<script>
	const obj = { v: 1 };
	export let x = obj.v;
</script>
<p>{x}</p>
