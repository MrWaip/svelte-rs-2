<svelte:options immutable />
<script>
	let n = 0;
</script>

<button on:click={() => n++}>{n}</button>