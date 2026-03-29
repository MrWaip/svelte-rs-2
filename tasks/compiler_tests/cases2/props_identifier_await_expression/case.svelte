<script>
	const props = $props()
</script>

{#await fetch(1, 2, 3, props.field1)}
{/await}