<script>
	import Child from './Child.svelte';
	let { value = $bindable() } = $props();
	const idx = 0;
</script>

<Child bind:value={value[idx].name} />
