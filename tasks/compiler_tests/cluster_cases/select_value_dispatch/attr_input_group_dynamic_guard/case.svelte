<script>
	let group = $state([]);
	let v = $state('x');
</script>

<input type="checkbox" bind:group={group} value={v} />
<button onclick={() => (v = 'y')}>swap</button>
