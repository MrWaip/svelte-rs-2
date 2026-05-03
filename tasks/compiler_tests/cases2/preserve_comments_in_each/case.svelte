<script>
	let { items } = $props();
</script>

{#each items as item}
	<!-- row -->
	<span>{item}</span>
{/each}
