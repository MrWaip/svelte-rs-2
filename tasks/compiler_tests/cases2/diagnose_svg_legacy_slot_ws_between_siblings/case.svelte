<script>
	import Child from './Child.svelte';
</script>

<Child>
	<svelte:fragment slot="header">
		<g><path d="M1"/></g>
		<g><path d="M2"/></g>
	</svelte:fragment>
</Child>
