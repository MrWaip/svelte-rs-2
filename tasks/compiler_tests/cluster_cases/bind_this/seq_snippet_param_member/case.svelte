<script>
</script>
{#snippet funBind(context)}
	<input bind:this={() => {}, (e) => context.element = e} />
{/snippet}
{@render funBind({ set element(e) {} })}
