<script>
	let size = $state(1);
	function bump() { size++; }
</script>
<button onclick={bump}>go</button>
<svelte:element this="{`h${size}`}">hello</svelte:element>
