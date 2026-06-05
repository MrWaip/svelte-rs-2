<svelte:options runes={false} />
<script>
	const k = "z"; export let { [k]: v } = { z: 1 };
</script>
<button>{v}</button>
