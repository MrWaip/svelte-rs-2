<script>
	import Child from './Child.svelte';
	import { CONST } from 'x';
	let { ref = $bindable() } = $props();
</script>

{#snippet foo(a)}<span>{CONST} {a}</span>{/snippet}

<Child bind:ref>{@render foo(1)}</Child>
