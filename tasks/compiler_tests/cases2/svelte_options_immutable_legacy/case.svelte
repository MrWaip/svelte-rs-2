<svelte:options immutable={true} />
<script>
	export let items = [1, 2];
	export let user = { name: 'a' };
</script>

<p>{items.length}</p>
<p>{user.name}</p>
