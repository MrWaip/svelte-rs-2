<script>
	let count = 0;

	async function compute(v) {
		return v * 2;
	}
</script>

{#snippet row()}
	<p>{await compute(count)}</p>
{/snippet}

{@render row()}
<button onclick={() => count++}>inc</button>
