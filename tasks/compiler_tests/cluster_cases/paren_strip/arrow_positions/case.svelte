<script>
	let a = $state(0)
	const handler = $derived(a ? (() => { a++ }) : undefined)
	const list = $derived([(() => a), (() => a + 1)])
	const cfg = $derived({ run: (() => a) })
	const call = $derived([1].map((() => a)))
</script>

<button onclick={handler}>{a} {list.length} {cfg.run()} {call.length}</button>
