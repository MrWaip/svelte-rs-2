<script>
let items = $state([1, 2, 3]);
export function getTotal() {
	const total = $derived.by(() => {
		let sum = 0;
		for (const item of items) {
			sum += item;
		}
		return sum;
	});
	return total;
}
</script>
