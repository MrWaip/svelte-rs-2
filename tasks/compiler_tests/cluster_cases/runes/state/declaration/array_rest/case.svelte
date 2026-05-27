<script>
	let [a, ...rest] = $state([1, 2, 3]);
</script>
<button>{a}{rest.length}</button>
