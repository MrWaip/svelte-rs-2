<script>
	let o = $state({ tag: 'div' });
	function bump() { o = { tag: 'p' }; }
</script>
<button onclick={bump}>go</button>
<svelte:element this="{o.tag}">hello</svelte:element>
