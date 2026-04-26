<svelte:options runes={false} />
<script>
	export let foo;
	export let baz = undefined;
</script>

<p>{foo}{baz}</p>
