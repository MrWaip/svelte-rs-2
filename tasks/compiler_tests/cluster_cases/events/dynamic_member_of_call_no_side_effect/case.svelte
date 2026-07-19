<script>
	let make = $state((name) => ({ handler: () => console.log(name) }));
</script>

<button onclick={make("Tama").handler}>go</button>
