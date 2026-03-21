<script>
	let { pairs = [] } = $props();
</script>

{#each pairs as [key, val]}
	<p>{key}={val}</p>
{/each}