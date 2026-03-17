<script>
	import { fly } from 'svelte/transition';
	let y = $state(200);
</script>

<div transition:fly={{ y }}>hello</div>
