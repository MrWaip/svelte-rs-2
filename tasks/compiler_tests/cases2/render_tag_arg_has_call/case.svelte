<script>
	function getData() { return [1, 2, 3]; }
</script>

{#snippet show(data)}
	<p>{data}</p>
{/snippet}

{@render show(getData())}