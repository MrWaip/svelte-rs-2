<script>
	import { fade } from 'svelte/transition';
	let visible = $state(true);
</script>

{#if visible}
	<div transition:fade>hello</div>
{/if}
