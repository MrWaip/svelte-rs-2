<script>
	let gate = $state(true);
	let loaded = $derived(await gate);
</script>

<button onclick={() => (gate = !gate)}>toggle</button>
{#if gate}yes{/if}
<p>{loaded}</p>
