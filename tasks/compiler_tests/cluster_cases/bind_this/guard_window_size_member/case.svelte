<script>
	let obj = { w: 0 };
</script>
<svelte:window bind:innerWidth={obj.w}/>
<div>{obj.w}</div>
