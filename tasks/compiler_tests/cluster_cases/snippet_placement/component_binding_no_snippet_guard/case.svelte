<script>
	import Child from './Child.svelte';
	let { ref = $bindable(), items } = $props();
</script>

<Child bind:ref>{items}</Child>
