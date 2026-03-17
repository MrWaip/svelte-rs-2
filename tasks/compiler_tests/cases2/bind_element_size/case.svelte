<script>
	let w = $state(0);
	let h = $state(0);
</script>

<div bind:clientWidth={w} bind:clientHeight={h}></div>

<div bind:offsetWidth={w} bind:offsetHeight={h}></div>
