<script module>
	const shared = await Promise.resolve(1);
</script>

<script>
	let count = 0;
</script>

<p>{shared} {count}</p>
<button onclick={() => count++}>inc</button>
