<script>
	// svelte-ignore await_waterfall
	let { name, age } = $derived(await fetch('/api'));
</script>

<p>{name} {age}</p>
