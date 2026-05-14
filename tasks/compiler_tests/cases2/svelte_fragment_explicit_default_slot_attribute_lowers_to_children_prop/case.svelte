<script>
	import Outer from './Outer.svelte';
	let { children } = $props();
</script>

<Outer>
	<svelte:fragment slot="default">
		{@render children?.()}
	</svelte:fragment>
</Outer>
