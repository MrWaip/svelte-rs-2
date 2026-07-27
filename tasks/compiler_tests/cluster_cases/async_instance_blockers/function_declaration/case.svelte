<script>
	let gate = $state(0);
	let loaded = $derived(await gate);
	let after = gate + 1;

	function reader() {
		return after;
	}
</script>

<button onclick={() => gate++}>inc</button>
<p>{reader()}</p>
