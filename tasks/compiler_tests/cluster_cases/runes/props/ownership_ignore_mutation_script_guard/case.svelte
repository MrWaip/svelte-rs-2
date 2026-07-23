<script>
	let { test } = $props();
	function go() {
		//svelte-ignore ownership_invalid_mutation
		test.test = Math.random();
	}
</script>

<button onclick={go}></button>
