<script>
	let gate = $state(0);
	let loaded = $derived(await gate);
	const uid = $props.id();
	let after = gate + 1;
</script>

<button onclick={() => gate++}>inc</button>
<p>{uid}{after}</p>
