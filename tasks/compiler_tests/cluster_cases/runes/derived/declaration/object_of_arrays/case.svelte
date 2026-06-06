<script>
	let x = $state({p: [1, 2], q: [3, 4]});
	let {p: [a, b], q: [c, d]} = $derived(x);
</script>
<button>{a}{b}{c}{d}</button>