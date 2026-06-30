<script>
	class Box {
		value = $state(0);
	}
	const box = new Box();
</script>

<button onclick={() => box.value++}>{box.value}</button>
