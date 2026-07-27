<script>
	let { p } = $props();
	const { first, second } = $derived(await p);
</script>

<p>{first} {second}</p>
