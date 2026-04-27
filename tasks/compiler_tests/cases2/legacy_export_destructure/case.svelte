<svelte:options runes={false} />
<script>
	export let { x: foo = 'default-x', z: [bar] } = { x: 'a', z: ['b'] };
</script>

<p>{foo}{bar}</p>
