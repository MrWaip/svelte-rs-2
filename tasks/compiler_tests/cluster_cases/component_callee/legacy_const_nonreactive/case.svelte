<svelte:options runes={false} />
<script>
	import Foo from './Foo.svelte';
</script>
{#if Foo}
	{@const Component = Foo}
	<Component />
{/if}
