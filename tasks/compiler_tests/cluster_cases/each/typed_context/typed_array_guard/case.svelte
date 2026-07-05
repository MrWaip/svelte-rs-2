<script lang="ts">
	let items = $state([[1]]);
</script>
<button onclick={() => items.push([items.length])}>add</button>
{#each items as [a]: number[]}
	<span>{a}</span>
{/each}
