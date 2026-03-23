<script>
let count = $state(0);
export function getDoubled() {
	const doubled = $derived(count * 2);
	return doubled;
}
</script>
