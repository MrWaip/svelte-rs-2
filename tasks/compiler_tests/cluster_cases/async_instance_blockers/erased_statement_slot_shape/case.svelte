<script>
	let gate = $state(0);
	let loaded = $derived(await gate);
	$inspect(loaded);
</script>

<button onclick={() => gate++}>inc</button>
<p>{loaded}</p>
