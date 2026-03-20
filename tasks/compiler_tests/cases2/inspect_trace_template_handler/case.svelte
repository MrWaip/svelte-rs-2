<script>
	let count = $state(0);
</script>

<button onclick={() => {
	$inspect.trace();
	count++;
}}>Click</button>
