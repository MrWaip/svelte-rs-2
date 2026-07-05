<script>
	let foo = $state(0);
	let bar = $derived(foo + 1);
</script>

<button onclick={() => foo++}>{foo} {bar}</button>
