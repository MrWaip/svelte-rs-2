<script>
	let w = $state(0);
</script>

<svelte:window bind:innerWidth={w} />
<div>{w}</div>
