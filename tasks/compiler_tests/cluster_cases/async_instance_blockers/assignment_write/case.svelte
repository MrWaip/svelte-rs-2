<script>
	let gate = $state(0);
	let plain = 1;
	let loaded = $derived(await gate);
	plain = 2;
</script>

<button onclick={() => gate++}>inc</button>
<p>{plain}</p>
<p>{loaded}</p>
