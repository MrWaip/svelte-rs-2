<script>
	import Inner from './Inner.svelte';
	export let flag;
	$: Holder = { component: Inner };
</script>

{#if flag}
	<Holder.component />
{/if}
