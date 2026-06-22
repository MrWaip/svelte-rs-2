<script>
	export let x;
</script>

{#if "Eva".startsWith('E')}
	eee
{:else if x}
	def
{:else}
	rrr
{/if}