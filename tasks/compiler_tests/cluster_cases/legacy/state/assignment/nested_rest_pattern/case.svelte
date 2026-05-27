<script>
	let x = 0;
	let z = 0;
	let arr = [1, 2, 3];
	function update() {
		[x, ...{ z = 26 }] = arr;
	}
</script>
<button on:click={update}>{x}{z}</button>
