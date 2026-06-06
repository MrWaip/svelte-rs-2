<script>
	let x = $state([1, 2, 3]);
	let [a, ...[b, c]] = $derived(x);
</script>
<button>{a}{b}{c}</button>