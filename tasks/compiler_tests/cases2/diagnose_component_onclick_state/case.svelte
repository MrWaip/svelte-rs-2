<script>
	import Button from './Button.svelte';
	let count = $state(0);
</script>

<Button theme="primary" onclick={() => count++}>Clicked {count} times</Button>
