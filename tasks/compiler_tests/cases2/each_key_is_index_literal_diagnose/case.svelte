<script>
	const facts = [
		'Cats have five toes on their front paws, but only four on the back.',
		"A group of flamingos is called a 'flamboyance'.",
		"Bananas are berries, but strawberries aren't."
	];
</script>

<ol>
	{#each facts as fact, i (i)}
		<li>{fact}</li>
	{/each}
</ol>
