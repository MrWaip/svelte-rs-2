<script>
	let r = $state(/ab/);
</script>

<button onclick={() => (r = /cd/)}>x</button>
<p>{r}</p>
