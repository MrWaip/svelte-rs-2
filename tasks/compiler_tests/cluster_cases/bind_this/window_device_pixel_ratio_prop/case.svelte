<script>
	export let dpr;
</script>
<svelte:window bind:devicePixelRatio={dpr}/>
<div>{dpr}</div>
