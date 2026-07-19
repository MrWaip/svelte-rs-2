<script>
	let { foo } = $props();
</script>

<button onclick={foo}>a</button>
<button on:click={foo}>b</button>
