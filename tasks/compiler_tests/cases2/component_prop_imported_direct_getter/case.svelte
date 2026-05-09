<script>
	import Comp from './Comp.svelte';
	import { foo } from './x.js';
	let { x } = $props();
</script>

<Comp p={foo} />
