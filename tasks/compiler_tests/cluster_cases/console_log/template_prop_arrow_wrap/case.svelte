<script>
	import Child from './Child.svelte';
</script>

<Child onSelect={({ detail }) => console.log("selected", detail)} />
