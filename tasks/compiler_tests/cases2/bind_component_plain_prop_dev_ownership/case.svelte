<script>
	import Child from "./Child.svelte";
	let { value } = $props();
</script>

<Child bind:value />
