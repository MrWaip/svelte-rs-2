<script>
	let deferred = $state(Promise.withResolvers());
</script>

<svelte:element this={await deferred.promise}>hello</svelte:element>
