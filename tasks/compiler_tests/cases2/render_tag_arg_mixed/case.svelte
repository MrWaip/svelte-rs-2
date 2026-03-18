<script>
	let name = $state('world');
	function greet() { return 'hello'; }
</script>

{#snippet show(greeting, person)}
	<p>{greeting} {person}</p>
{/snippet}

{@render show(greet(), name)}