<script>
	let [a, ...[b, c]] = $state([1, 2, 3]);
</script>
<button>{a}{b}{c}</button>
