<script>
    export let rows;

    function check(key) {
        return key === 'a';
    }
</script>

{#each rows as row (row.key)}
    {#if check(row.key)}<p>ok</p>{/if}
{/each}
