<script>
	let deferred = $state(Promise.withResolvers());
</script>

<h1>{@html await deferred.promise}</h1>
