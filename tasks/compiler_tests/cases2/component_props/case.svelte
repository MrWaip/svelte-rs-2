<script>
	import Button from './Button.svelte';
	let count = $state(0);
	function handler() { count++; }
</script>

<Button label="Click me" onclick={handler} count={count} />
