<script>
	import Child from './Child.svelte';
	let { items } = $props();
</script>

{#snippet foo(a)}<span>{items} {a}</span>{/snippet}

<Child>{@render foo(1)}</Child>
