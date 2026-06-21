<svelte:options immutable />
<script>
	export let x = 1;
	$: doubled = x * 2;
</script>

<p>{doubled}</p>