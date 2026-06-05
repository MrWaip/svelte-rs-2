<svelte:options runes={false} />
<script>
	export let { p: [a, b], q: [c, d] } = { p: [1, 2], q: [3, 4] };
</script>
<button>{a}{b}{c}{d}</button>
