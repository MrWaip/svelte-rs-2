<script>
	import Child from "./Child.svelte";
	let { foo = $bindable() } = $props();
</script>

<Child bind:value={foo} />
