<svelte:options immutable={true} />
<script>
	export let items = [1, 2];
</script>

<p>{items.length}</p>
