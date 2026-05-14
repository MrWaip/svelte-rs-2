<script>
    const store = { items: ['a', 'b'], data: { a: 1, b: 2 } };
</script>

{#each store.items as item}
    <input bind:value={store.data[item]} />
{/each}
