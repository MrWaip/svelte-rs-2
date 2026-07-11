<script>
	import Child from './Child.svelte';
	let { items } = $props();
</script>

<Child disabled={!items.length} --my-var="baseline">
	{#snippet element({ idx })}
		<div>{idx}</div>
	{/snippet}
</Child>
