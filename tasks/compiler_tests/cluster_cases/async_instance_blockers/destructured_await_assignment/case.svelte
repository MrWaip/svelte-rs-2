<script>
	let gate = $state(0);
	let { first, second } = await Promise.resolve({ first: gate, second: 2 });
</script>

<button onclick={() => gate++}>inc</button>
<p>{first}{second}</p>
