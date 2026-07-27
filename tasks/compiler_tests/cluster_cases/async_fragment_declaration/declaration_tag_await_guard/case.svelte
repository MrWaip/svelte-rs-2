<script>
	let n = $state(1);
</script>

{const a = await Promise.resolve(n)}
<p>{a}</p>
<button onclick={() => n++}>go</button>
