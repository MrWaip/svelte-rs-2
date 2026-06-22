<svelte:options runes={false} />
<script>
	import Widget from './Widget.svelte';
	export let widgets = [{ name: 'foo' }];
</script>
{#each widgets as widget, i}
	<Widget widget='{widget}' index='{i}'/>
{/each}
