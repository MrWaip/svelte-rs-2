<script>
	import { writable } from 'svelte/store';
	const count = writable(0);
</script>

{#snippet foo()}
	{$count}
{/snippet}

{@render foo()}
