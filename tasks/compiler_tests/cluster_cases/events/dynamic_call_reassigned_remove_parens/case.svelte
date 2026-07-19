<script>
	let makeHandler = null;
	makeHandler = () => () => console.log("x");
</script>

<button on:click={makeHandler()}>x</button>
