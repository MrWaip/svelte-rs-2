<script>
	let count = $state(0);
</script>

{#if count}
	{@debug count}
	<button onclick={() => count++}>+</button>
{/if}
