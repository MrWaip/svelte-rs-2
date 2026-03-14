<script>
	let { items = [] } = $props();
</script>

{#each items as item}
    {item}

    <div>
		{item} + example
	</div>
{/each}