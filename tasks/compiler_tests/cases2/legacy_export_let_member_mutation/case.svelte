<svelte:options runes={false} />
<script>
	export let obj = { a: 1 };
	obj.a = 99;
</script>

<p>{obj.a}</p>
