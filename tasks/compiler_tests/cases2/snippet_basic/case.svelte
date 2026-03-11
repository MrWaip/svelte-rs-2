<script>
	let message = $state("hello");
</script>

{#snippet greeting(name)}
	<p>Hello {name}</p>
{/snippet}

{@render greeting(message)}
