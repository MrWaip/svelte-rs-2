<script>
	export let a;
	let count = 0;
	function bump() {
		count += 1;
	}
</script>

<div title={[() => $$props.x]}>{a}</div>
<button onclick={bump}>{count}</button>
