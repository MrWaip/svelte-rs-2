<script>
	let value = $state("x");
</script>

<div foo="a{value}"></div>
<button onclick={() => (value += "!")}>go</button>