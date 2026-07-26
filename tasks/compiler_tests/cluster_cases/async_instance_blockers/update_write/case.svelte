<script>
	let gate = $state(0);
	let counter = 1;
	let loaded = $derived(await gate);
	counter++;
</script>

<button onclick={() => gate++}>inc</button>
<p>{counter}</p>
<p>{loaded}</p>
