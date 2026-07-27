<script>
	export let base = 0;
	let count = 0;
	$: count = base * 2;
	$: $count = 1;
</script>

<button onclick={() => count++}>increment</button>
