<script>
	async function first() {
		return false;
	}

	async function second() {
		return true;
	}
</script>

{#if await first()}
	<p>first</p>
{:else if await second()}
	<p>second</p>
{:else}
	<p>fallback</p>
{/if}
