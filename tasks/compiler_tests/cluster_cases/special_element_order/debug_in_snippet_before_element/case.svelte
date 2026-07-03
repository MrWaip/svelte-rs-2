<script>
	let count = $state(0);
</script>

{#snippet row()}
	{@debug count}
	<button onclick={() => count++}>+</button>
{/snippet}

{@render row()}
