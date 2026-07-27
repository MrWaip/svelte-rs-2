<script>
	let n = $state(1);
</script>

<div>
	{const a = await Promise.resolve(n)}
	<p>{a}</p>
</div>
<button onclick={() => n++}>go</button>
