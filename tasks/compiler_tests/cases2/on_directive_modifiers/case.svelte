<script>
	function handleClick() {
		console.log('clicked');
	}
</script>

<button on:click|preventDefault={handleClick}>Click me</button>
