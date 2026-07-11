<svelte:options immutable={true} />
<script>
	export let n;
	let cache = {};
	function bump(i) { cache[i] = i; }
	$: visible = compute(cache, n);
</script>
<button on:click={() => bump(1)}>{visible}</button>
