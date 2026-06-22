<script>
	export let scrollY;
</script>
<svelte:window bind:scrollY={scrollY}/>
<div>{scrollY}</div>
