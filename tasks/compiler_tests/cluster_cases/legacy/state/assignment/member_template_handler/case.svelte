<script>
	let x = 0;
</script>
<button on:click={() => ({ x } = { x: 1 })}>obj</button>
<button on:click={() => ([x] = [2])}>arr</button>
<p>x: {x}</p>