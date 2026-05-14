<script>
	import Comp from './Comp.svelte';
	import { foo } from './x.js';
	let { x } = $props();
	const editUrl = foo();
</script>

<Comp href={editUrl} radius={16} />
