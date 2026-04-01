<script>
	async function process(items) {
		// svelte-ignore await_reactivity_loss
		for await (const item of items) {
			console.log(item);
		}
	}
</script>
