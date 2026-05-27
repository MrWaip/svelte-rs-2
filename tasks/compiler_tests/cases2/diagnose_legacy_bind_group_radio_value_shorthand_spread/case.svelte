<svelte:options runes={false} />
<script>
	export let value;
	export let group;
</script>

<input type="radio" bind:group {value} {...$$restProps} />
