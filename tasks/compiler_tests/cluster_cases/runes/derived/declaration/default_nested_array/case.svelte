<script>
	let x = $state([[1, 2], 3]);
	let [[a, b] = [9, 9], c] = $derived(x);
</script>
<button>{a}{b}{c}</button>