<script>
	import Container from './Container.svelte';
	import Child from './Child.svelte';
	let { value = $bindable() } = $props();
</script>

<Container let:idx>
	<Child bind:value={value[idx].name} />
</Container>
