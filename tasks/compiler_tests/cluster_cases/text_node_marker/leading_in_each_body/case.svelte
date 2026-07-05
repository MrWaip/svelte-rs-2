<script>
	let array = $state(['A']);
</script>

{#each array as a}{a}<br/>{/each}
<button onclick={() => array.push('B')}>add</button>
