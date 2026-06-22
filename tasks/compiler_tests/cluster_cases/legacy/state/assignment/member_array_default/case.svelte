<script>
	const a = 100;
	const arr = [{ a: 1 }, 2];
	function go() {
		[arr[0].a, arr[1] = a] = [arr[1]];
	}
</script>
<button on:click={go}>{JSON.stringify(arr)}</button>