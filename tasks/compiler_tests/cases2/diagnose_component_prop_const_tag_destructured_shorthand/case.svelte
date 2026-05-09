<script>
	import Comp from './Comp.svelte';
	import { writable } from 'svelte/store';
	const value = writable({ a: 1, b: 2 });
</script>

{#if $value}
	{@const { a, b } = $value}
	<Comp x={a} {b} />
{/if}
