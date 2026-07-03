<script>
	let state = 5;
	let foo = $state(0);
</script>

<button onclick={() => foo++}>{foo} {state}</button>
