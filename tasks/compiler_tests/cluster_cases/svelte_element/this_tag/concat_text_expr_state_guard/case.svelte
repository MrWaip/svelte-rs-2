<script>
	let n = $state(1);
	function bump() { n++; }
</script>
<button onclick={bump}>go</button>
<svelte:element this="h{n}">hello</svelte:element>
