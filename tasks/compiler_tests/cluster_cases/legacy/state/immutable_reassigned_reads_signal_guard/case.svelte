<svelte:options immutable={true} />
<script>
	export let n;
	let total = 0;
	function bump() { total = total + 1; }
	$: visible = compute(total, n);
</script>
<button on:click={bump}>{visible}</button>
