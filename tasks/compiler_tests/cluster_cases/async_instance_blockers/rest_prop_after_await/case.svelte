<script>
	const x = await Promise.resolve(1);
	let { a, b = 2, c = $bindable(3), ...rest } = $props();
	function inc(){ c++; }
</script>
<button onclick={inc}>{x} {a} {b} {c} {JSON.stringify(rest)}</button>
