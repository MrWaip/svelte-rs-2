<script>
	let { initial = "a" } = $props();
	let selected = $state(null);
	let dyn_val = $state(initial);
	function rotate() {
		dyn_val = dyn_val + "!";
	}
</script>

<input type="radio" bind:group={selected} value={`item-${dyn_val}`} />
<button onclick={rotate}>rotate</button>
