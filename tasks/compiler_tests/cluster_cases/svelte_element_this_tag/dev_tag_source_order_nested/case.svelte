<script>
	let show = $state(false);
</script>

<svelte:element this={'p'}>before</svelte:element>
{#if show}
	<svelte:element this={'strong'}>during</svelte:element>
{/if}
<svelte:element this={'p'}>after</svelte:element>
