<script>
	import Outer from './Outer.svelte';
	export let name;
</script>

<Outer>
	<svelte:fragment>hello {name}</svelte:fragment>
</Outer>
