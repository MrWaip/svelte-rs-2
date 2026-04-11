<svelte:options runes={true} />

<script>
	import Widget from './Widget.svelte';

	let refs = $state([]);
	const Derived_1 = $derived(Widget);
</script>

<Derived_1 bind:this={refs[1]} />

{#if true}
	{@const Const_0 = Widget}
	<Const_0 bind:this={refs[0]} />
{/if}
