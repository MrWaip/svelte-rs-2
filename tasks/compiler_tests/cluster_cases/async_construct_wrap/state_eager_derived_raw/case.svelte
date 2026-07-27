<script>
	let count = $state(0);
	const delayed = $derived(await Promise.resolve(count));
	const doubled = $derived(count * 2);
</script>

<button onclick={() => count++}>inc</button>
<p>{delayed}{$state.eager(doubled) !== doubled}</p>
