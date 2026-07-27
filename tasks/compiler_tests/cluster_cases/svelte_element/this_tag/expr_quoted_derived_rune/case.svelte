<script>
	let n = $state(1);
	let tag = $derived('h' + n);
	function bump() { n++; }
</script>
<button onclick={bump}>go</button>
<svelte:element this="{tag}">hello</svelte:element>
