<script>
	let thisBug = $state();
</script>

<form bind:this={thisBug}>
	{#snippet Bug()}cool{/snippet}
</form>

{typeof thisBug}
