<script>
	let count = $state(0);

	function is_even() {
		return count % 2 === 0;
	}
</script>

{#if is_even()}
	<p>even</p>
{:else}
	<p>odd</p>
{/if}
