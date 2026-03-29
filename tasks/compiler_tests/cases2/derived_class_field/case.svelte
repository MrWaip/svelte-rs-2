<script>
	class Box {
		width = $state(0);
		height = $state(0);
		area = $derived(this.width * this.height);
	}
	let box = new Box();
</script>

<p>{box.area}</p>
