<script>
	let gate = $state(0);
	let loaded = $derived(await gate);
	let after = gate + 1;
</script>

<button onclick={() => gate++}>inc</button>
<p>{[1].map(() => after).join("")}</p>
