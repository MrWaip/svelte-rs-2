<script>
	let a = $state(0)
	const plain = $derived((function () { return a }(a)))
	const named = $derived((function f() { return a }(a)))
	const asyncf = $derived((async function () { return a }(a)))
	const memb = $derived((function () { return { x: a } }(a)).x)
</script>

<button onclick={() => a++}>{plain} {named} {typeof asyncf} {memb}</button>
