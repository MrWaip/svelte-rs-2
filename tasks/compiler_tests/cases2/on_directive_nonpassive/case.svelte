<script>
	function handleMove() {
		console.log('move');
	}
</script>

<div on:touchmove|nonpassive={handleMove}>Touch</div>
