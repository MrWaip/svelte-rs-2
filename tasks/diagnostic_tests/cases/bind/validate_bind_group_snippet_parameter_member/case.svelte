<script>
    let options = $state({ value: '' });
</script>
{#snippet field(item)}
    <input type="checkbox" bind:group={item.value} />
{/snippet}
{@render field(options)}
