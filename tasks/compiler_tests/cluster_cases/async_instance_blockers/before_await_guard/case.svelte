<script>
	let gate = $state(0);
	let before = 1;
	let loaded = $derived(await gate);
</script>

<button onclick={() => gate++}>inc</button>
<p>{before}</p>
