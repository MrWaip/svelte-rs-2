<script>
	let tags = $state(['div', 'span']);
	function bump() { tags = ['p']; }
</script>
<button onclick={bump}>go</button>
{#each tags as t}
	<svelte:element this="{t}">hello</svelte:element>
{/each}
