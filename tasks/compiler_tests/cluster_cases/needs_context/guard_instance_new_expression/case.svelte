<script>
	let n = $state(0);
	function make() { return new Date(); }
</script>
<button onclick={() => { n++; make(); }}>{n}</button>
