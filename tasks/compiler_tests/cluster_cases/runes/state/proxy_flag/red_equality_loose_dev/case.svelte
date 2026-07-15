<script>
	let { a, b } = $props();
	let eq = $state(a != b);
	function toggle() {
		eq = !eq;
	}
</script>

<button onclick={toggle}>{eq}</button>
