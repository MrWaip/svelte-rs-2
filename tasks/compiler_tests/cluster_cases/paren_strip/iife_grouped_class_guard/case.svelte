<script>
	let a = $state(0)
	const c = $derived((class { }(a)))
</script>

<button onclick={() => a++}>{typeof c}</button>
