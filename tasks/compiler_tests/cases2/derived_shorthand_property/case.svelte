<script>
let value = $state(0);
export function getInfo() {
	const computed = $derived(value * 2);
	return { computed };
}
</script>
