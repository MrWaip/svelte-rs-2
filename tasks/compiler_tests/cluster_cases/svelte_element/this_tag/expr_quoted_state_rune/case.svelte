<script>
	let tag = $state('div');
	function bump() { tag = 'p'; }
</script>
<button onclick={bump}>go</button>
<svelte:element this="{tag}">hello</svelte:element>
