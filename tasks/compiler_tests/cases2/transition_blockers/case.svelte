<script>
	import { fade } from 'svelte/transition';

	let data = await fetch('/api');
	let params = $state(data.params);
</script>

{#if true}
	<div transition:fade={params}>hello</div>
{/if}
