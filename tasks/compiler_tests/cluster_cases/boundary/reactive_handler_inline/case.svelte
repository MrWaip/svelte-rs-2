<script>
	let error = $state();
</script>

<svelte:boundary onerror={(e) => error = e}>x</svelte:boundary>

{#if error}<p>err</p>{/if}
