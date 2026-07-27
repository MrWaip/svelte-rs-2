<script>
	let n = $state(1);
</script>

{let a = await Promise.resolve(n), b = n + 1}
<p>{a}{b}</p>
<button onclick={() => n++}>go</button>
