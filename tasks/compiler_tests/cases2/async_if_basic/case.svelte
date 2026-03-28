<script>
	async function check() {
		return true;
	}
</script>

{#if await check()}
	<p>yes</p>
{/if}
