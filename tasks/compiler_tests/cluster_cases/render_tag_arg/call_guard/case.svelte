<script>
	let { children } = $props();
	let count = $state(0);
	function label(n) {
		return n + 1;
	}
</script>

<button onclick={() => count++}>go</button>
{@render children(label(count))}
