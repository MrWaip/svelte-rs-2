<script>
	import Child from './Child.svelte';
	let { data } = $props();
</script>
{#if data}
	{@const simpleReactive = data.foo}
	{@const { destr } = { destr: 1 }}
	{@const simpleStatic = 5}
	<Child a={simpleReactive} b={destr} c={simpleStatic} />
{/if}
