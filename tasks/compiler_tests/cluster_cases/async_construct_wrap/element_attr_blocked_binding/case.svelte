<script>
	import Child from './Child.svelte';

	await Promise.resolve();
	const x = 2;
</script>

<div title={x} class="a{x}b"></div>
<Child a={() => x} b={{ k: x }} />
