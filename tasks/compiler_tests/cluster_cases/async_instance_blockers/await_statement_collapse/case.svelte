<script>
	let gate = $state(0);
	let sink = 0;
	await gate;
	sink = gate + 1;
</script>

<button onclick={() => gate++}>inc</button>
<p>{sink}</p>
