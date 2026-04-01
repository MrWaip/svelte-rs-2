<script>
	let name = $state("world");
</script>

{#snippet greeting(msg)}
	<p>Hello {msg}</p>
{/snippet}

<p>{name}</p>
