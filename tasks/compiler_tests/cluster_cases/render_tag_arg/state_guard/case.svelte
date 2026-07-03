<script>
	let { children } = $props();
	let count = $state(0);
</script>

<button onclick={() => count++}>{count}</button>
{@render children(count)}
