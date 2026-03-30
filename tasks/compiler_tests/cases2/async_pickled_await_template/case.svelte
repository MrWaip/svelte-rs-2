<script>
	let promise = Promise.resolve('hello');
	const suffix = '!';
</script>

<p>{await promise + suffix}</p>
