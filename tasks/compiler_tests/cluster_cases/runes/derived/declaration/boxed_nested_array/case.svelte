<script>
	let x = $state([[1, 2], [3, 4]]);
	let [[a, b], [c, d]] = $derived(x.slice());
</script>
<button>{a}{b}{c}{d}</button>