<script>
	let gate = $state(0);
	let sink = 0;
	let loaded = $derived(await gate);
	sink = 1;
	sink = 2;
	let single = gate + 1;
</script>

<button onclick={() => gate++}>inc</button>
<p>{sink}{single}{loaded}</p>
