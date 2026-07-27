<script>
	let gate = $state(0);
	(await gate);
	let after = gate + 1;
</script>
<button onclick={() => gate++}>inc</button>
<p>{after}</p>
