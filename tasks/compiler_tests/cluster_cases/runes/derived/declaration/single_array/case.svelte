<script>
	let x = $state([1, 2]);
	let [a, b] = $derived(x);
</script>
<button>{a}{b}</button>