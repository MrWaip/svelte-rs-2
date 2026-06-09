<svelte:options runes={false} />
<script>
	let className = 'btn';
	export { className as class };
	export function getClass() {
		return className;
	}
</script>

<p>{className}</p>
