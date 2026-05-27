<script>
	let x = $state({a: 1, b: 2});
	let {a, b} = $derived(x);
</script>
<button>{a}{b}</button>