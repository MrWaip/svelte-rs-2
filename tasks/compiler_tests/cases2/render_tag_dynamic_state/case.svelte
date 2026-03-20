<script>
	let show = $state(null);
</script>

{#snippet greeting(name)}
	<p>Hello {name}</p>
{/snippet}

{@render show()}
