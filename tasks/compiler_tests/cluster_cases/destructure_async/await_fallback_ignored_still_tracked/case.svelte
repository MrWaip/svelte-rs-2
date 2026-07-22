<script>
	let f = $state(0);

	async function go() {
		// svelte-ignore await_reactivity_loss
		[f = false || await Promise.resolve(6)] = [];
	}
</script>

<button onclick={go}>{f}</button>
