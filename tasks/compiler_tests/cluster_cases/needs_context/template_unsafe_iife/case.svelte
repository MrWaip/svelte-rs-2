<script>
	let n = $state(0);
</script>
<button onclick={() => n++}>{(() => n)()}</button>
