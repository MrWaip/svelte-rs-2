<script>
	let { h = $bindable(0) } = $props();
</script>

<div bind:clientHeight={h}></div>
