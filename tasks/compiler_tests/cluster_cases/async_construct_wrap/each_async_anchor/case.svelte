<script>
	let deferred = $state(Promise.withResolvers());
</script>

<ul>
	{#each await deferred.promise as item}
		<li>{item}</li>
	{/each}
</ul>
