<script lang="ts">
	let count = $state(0);
</script>
<button onclick={() => count++}>{count}</button>
{#snippet row(c)}
	<span>{c}</span>
{/snippet}
{@render row(count)}
