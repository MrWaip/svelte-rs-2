<script>
let value = $state(0);
export function makeAccessor() {
	const computed = $derived(value + 1);
	return {
		get computed() {
			return computed;
		}
	};
}
</script>
