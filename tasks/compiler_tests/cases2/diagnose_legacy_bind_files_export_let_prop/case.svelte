<svelte:options runes={false} />
<script>
	export let files = undefined;
</script>

<input type="file" bind:files multiple />
