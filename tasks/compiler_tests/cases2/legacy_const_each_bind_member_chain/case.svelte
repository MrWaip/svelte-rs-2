<script>
    const obj = { keys: ['a'], fields: { a: { value: 0 } } };
</script>

{#each obj.keys as key (key)}
    {@const field = obj.fields[key]}
    <input bind:value={field.value} />
{/each}
