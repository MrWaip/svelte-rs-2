<script>
	let gate = $state(0);
	let loaded = $derived(await gate);
	let first = gate + 1, second = gate + 2;
</script>

<button onclick={() => gate++}>inc</button>
<p>{loaded}{first}{second}</p>
