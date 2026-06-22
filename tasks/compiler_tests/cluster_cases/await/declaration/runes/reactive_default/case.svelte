<script>
	let { object } = $props();
	let num = $state(0);
	function inc() {
		num++;
	}
</script>
<button onclick={inc}>inc</button>
{#await object then { v = num }}
	<button>{v} {num}</button>
{/await}
