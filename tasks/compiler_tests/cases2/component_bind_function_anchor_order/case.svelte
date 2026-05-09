<script>
	import Comp from './Comp.svelte';
	let value = $state(0);
</script>

<div>
	<Comp bind:value={() => value, (v) => value = v} />
</div>
