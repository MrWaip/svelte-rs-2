<script>
	let gate = $state(0);
	let loaded = $derived(await gate);

	class Box {
		value = 1;
	}

	let box = new Box();
</script>

<button onclick={() => gate++}>inc</button>
<p>{loaded}{box.value}</p>
