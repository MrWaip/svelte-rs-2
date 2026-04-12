<svelte:options runes={false} />
<script>
	export let { x: foo, z: [bar] } = { x: 'a', z: ['b'] };
</script>

<p>{foo}{bar}</p>
