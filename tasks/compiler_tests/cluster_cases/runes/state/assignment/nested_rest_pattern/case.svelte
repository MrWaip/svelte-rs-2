<script>
	let x = $state(0);
	let z = $state(0);
	let arr = [1, 2, 3];
	function update() {
		[x, ...{ z = 26 }] = arr;
	}
</script>
<button onclick={update}>{x}{z}</button>
