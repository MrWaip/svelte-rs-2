<script>
	import { fade } from 'svelte/transition';

	let x = $state(false);
	let y = $state(true);
</script>

{#if x}
	<p>first</p>
{:else if y}
	<div transition:fade>second</div>
{/if}
