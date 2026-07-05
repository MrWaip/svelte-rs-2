<script lang="ts">
	let items = $state([1, 2]);
</script>
<button onclick={() => items.push(items.length)}>add</button>
{#each items as item: number}
	<span>{item}</span>
{/each}
