<script>
	import Comp from './Comp.svelte';
	let { x } = $props();
</script>

<Comp url={import.meta.env.VITE_X} />
