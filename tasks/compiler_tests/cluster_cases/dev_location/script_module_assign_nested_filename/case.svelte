<script module>
	const cache = {};
	export function fill(items) {
		items.forEach((item) => cache[item.id] = item);
	}
</script>

<button onclick={() => fill([])}>go</button>
