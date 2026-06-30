<script lang="ts">
	let items = $state([{ a: 1 }]);
</script>
<button onclick={() => items.push({ a: items.length })}>add</button>
{#each items as { a }: { a: number }}
	<span>{a}</span>
{/each}
