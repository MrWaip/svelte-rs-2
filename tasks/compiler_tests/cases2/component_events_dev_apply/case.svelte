<script>
	import Widget from './Widget.svelte';

	let count = $state(0);

	function getHandler() {
		return () => count++;
	}
</script>

<Widget on:done={getHandler()} />
