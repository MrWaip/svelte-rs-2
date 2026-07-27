<script>
	let gate = $state(0);
	let loaded = $derived(await gate);
	$inspect(loaded);
	let after = gate + 1;
</script>

<button onclick={() => gate++}>inc</button>
<p>{loaded}{after}</p>
