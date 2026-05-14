<script>
	import Comp from './Comp.svelte';
	let value = $state(0);
</script>

<Comp bind:value={() => value, (v) => value = v} label="x" id="y" />
