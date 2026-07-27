<script>
	async function g() { return 1; }
	async function f(v) { return v; }
	let x = $derived(await f(await g()));
</script>
<p>{x}</p>
