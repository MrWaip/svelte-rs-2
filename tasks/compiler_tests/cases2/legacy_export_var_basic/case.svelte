<svelte:options runes={false} />
<script>
	export var count = 1;
	count = count + 1;
</script>

<p>{count}</p>
