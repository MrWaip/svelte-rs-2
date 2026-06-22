<script>
    let options = $state({ value: '' });
</script>
{#snippet field(item)}
    <input bind:value={item.value} />
{/snippet}
{@render field(options)}
