<script>
	import Child from './Child.svelte';
	let x = $state('x1');
</script>

<button onclick={() => (x = 'x2')}>x</button>
<Child {x} />
