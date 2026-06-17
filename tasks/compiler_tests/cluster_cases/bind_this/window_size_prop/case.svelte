<script>
	export let width;
</script>
<svelte:window bind:innerWidth={width}/>
<div>{width}</div>
