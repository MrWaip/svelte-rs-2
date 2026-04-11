<script>let items = $state([1, 2, 3]); let value = { nested: { current: 1 } };</script>
{#each items as item}
    {({ nested: { current: item } } = value)}
{/each}
