<script>
	let n = $state(1);
</script>

{let a = await Promise.resolve(n)}
{let b}
<p>{a}{b}</p>
<button onclick={() => (b = n)}>go</button>
