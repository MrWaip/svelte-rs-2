<script>
	import { writable } from 'svelte/store';
	let count = writable(0);
	count = writable(1);
</script>

{#snippet foo()}
	{$count}
{/snippet}

{@render foo()}
