<script>
	let x = $state({ a: 1 });
</script>

<button onclick={() => x.a++}>{x.a}</button>
