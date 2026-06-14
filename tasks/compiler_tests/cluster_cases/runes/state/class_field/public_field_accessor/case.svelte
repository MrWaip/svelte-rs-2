<script>
	class Box {
		value = $state(0);
		bump() {
			this.value++;
		}
	}
	const box = new Box();
</script>

<button onclick={() => box.bump()}>{box.value}</button>
