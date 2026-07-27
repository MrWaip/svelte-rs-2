<script>
	let gate = $state(0);
	let one = $derived(await gate);
	let sync1 = gate + 1;
	let sync2 = gate + 2;
	let two = $derived(await gate);
	let sync3 = gate + 3;
</script>

<button onclick={() => gate++}>inc</button>
<p>{one}</p>
<p>{sync1}{sync2}</p>
<p>{two}</p>
<p>{sync3}</p>
