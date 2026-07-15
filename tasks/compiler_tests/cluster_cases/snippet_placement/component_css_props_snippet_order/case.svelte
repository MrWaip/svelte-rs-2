<script>
	import Child from './Child.svelte';
	let { data } = $props();
</script>

<Child value={data} --my-var="baseline">
	{#snippet element({ idx })}
		<div>{idx}</div>
	{/snippet}
</Child>
