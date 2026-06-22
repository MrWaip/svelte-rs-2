<script>
	let fns = [() => {}];
	let n = $state(0);
</script>
<button onclick={() => { n++; fns.pop()(); }}>{n}</button>
