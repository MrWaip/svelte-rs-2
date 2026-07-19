<script>
	let x = $state(1);
</script>
<button onclick={() => x++}>{x}</button>
<div>
	{const x = 'inner'}
	<b>{x}</b>
</div>
