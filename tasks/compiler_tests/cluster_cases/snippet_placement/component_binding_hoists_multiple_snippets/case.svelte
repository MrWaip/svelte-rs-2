<script>
	import Child from './Child.svelte';
	let { ref = $bindable(), items } = $props();
</script>

{#snippet a(p)}<span>{items} {p}</span>{/snippet}
{#snippet b(q)}<em>{items} {q}</em>{/snippet}

<Child bind:ref>{@render a(1)}{@render b(2)}</Child>
