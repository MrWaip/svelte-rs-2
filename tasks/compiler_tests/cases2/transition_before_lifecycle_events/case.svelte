<script>
	import { fade } from 'svelte/transition';
	let animated = $state(false);
</script>

<div
	out:fade
	on:introstart={() => animated = true}
	on:outroend={() => animated = false}
></div>
