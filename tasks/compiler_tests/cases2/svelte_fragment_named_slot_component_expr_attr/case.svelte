<script>
	import Outer from './Outer.svelte';
	import Inner from './Inner.svelte';
	export let value;
</script>

<Outer>
	<svelte:fragment slot="content">
		<Inner prop={value} />
	</svelte:fragment>
</Outer>
