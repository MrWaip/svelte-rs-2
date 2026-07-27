<script>
	import Child from './Child.svelte';
	let { ref = $bindable(), items } = $props();
</script>

{#snippet foo(a)}<span>{items} {a}</span>{/snippet}

<Child bind:ref>{@render foo(2)}</Child>
