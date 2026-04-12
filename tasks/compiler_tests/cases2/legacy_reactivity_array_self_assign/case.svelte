<svelte:options runes={false} />
<script>
	let numbers = [1, 2, 3];

	function add() {
		numbers.push(numbers.length + 1);
		numbers = numbers;
	}
</script>

<p>{numbers.length}</p>
<button onclick={add}>add</button>
