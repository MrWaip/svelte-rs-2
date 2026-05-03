<script>
	let { cond } = $props();
</script>

{#if cond}
	<!-- inside if -->
	<span>visible</span>
{/if}
