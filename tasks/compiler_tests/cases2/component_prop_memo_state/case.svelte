<script>
	import Comp from './Comp.svelte';
	let i = $state(0);
	let index = $state(0);

	function bump() { i++; }
</script>

<Comp active={i === index} />
<button onclick={bump}>bump</button>