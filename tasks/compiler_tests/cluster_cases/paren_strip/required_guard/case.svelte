<script>
	let a = $state(0)
	const g1 = $derived(a && (() => a))
	const g2 = $derived(a ?? (() => a))
	const g3 = $derived(() => ({ x: a }))
	const g4 = $derived((a, () => a))
	const g5 = $derived((a + 1) * 2)
	const g6 = $derived((() => a)())
	const g7 = $derived((function () { return a })())
</script>

<button onclick={() => a++}>{g5} {g6} {g7} {typeof g1} {typeof g2} {g3().x} {g4()}</button>
