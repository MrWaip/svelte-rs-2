<script>
	let state = $state(0);
	let derived = $derived(state + 1);
</script>

<button onclick={() => state++}>{state} {derived}</button>
