<script>
	let { x } = $props();
</script>

<svelte:boundary>
	boundary {x} text

	{#snippet failed()}<p>f</p>{/snippet}
</svelte:boundary>
