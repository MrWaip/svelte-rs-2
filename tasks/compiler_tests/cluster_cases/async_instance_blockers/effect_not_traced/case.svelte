<script>
	let gate = $state(0);
	let shown = 1;
	let loaded = $derived(await gate);

	$effect(() => {
		shown = 2;
	});
</script>

<button onclick={() => gate++}>inc</button>
<p>{shown}</p>
