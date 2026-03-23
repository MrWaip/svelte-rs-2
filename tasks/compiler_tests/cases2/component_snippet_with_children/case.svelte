<script>
	let name = $state("world");
</script>

<Card>
	{#snippet title()}
		<h2>Hello</h2>
	{/snippet}
	<p>Content {name}</p>
</Card>
