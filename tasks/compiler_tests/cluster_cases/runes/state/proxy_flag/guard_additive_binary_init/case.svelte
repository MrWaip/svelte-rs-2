<script>
	let { a, b } = $props();
	let sum = $state(a + b);
	function inc() {
		sum = sum + 1;
	}
</script>

<button onclick={inc}>{sum}</button>
