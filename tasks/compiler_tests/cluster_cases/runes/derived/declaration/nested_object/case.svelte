<script>
	let x = $state({p: {a: 1}, q: {b: 2}});
	let {p: {a}, q: {b}} = $derived(x);
</script>
<button>{a}{b}</button>