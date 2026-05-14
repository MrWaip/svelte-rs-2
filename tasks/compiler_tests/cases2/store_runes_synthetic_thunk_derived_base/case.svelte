<script>
	let { manager } = $props();
	let store = $derived(manager.store);
</script>

<p>{$store}</p>
