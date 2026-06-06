<script>
	let src = $state({ "a-b": 1, "c d": 2 });
	let { "a-b": ab, "c d": cd } = $derived(src);
</script>
<button>{ab}{cd}</button>
