<script>
    const obj = { list: [1, 2, 3] };
</script>

{#each obj.list as item}
    <span>{item}</span>
{/each}
