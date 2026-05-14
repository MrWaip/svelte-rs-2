<script>
	import Comp from './Comp.svelte';
	import { obj } from './x.js';
	let { x } = $props();
	const y = obj.prop;
</script>

<Comp p={y} />
