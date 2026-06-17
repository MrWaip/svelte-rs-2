<script>
	let width = 0;
	$: doubled = width * 2;
</script>
<svelte:window bind:innerWidth={width}/>
<p>{doubled}</p>
