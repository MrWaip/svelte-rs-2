<svelte:options runes={false} />
<script>
	export let a = 1;
	export let b = a;
</script>
<p>{a}{b}</p>
