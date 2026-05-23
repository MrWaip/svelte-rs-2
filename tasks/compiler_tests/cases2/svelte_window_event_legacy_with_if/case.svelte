<script>
	function onClick() {}
	let show = true;
</script>

<svelte:window on:click={onClick} />

{#if show}
	<div>x</div>
{/if}
