<script>
	let count = 0;

	async function check(v) {
		return v > 0;
	}
</script>

{#if await check(count)}
	<p>yes</p>
{/if}
<button onclick={() => count++}>inc</button>
