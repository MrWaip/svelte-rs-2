<script>let items = $state([{ value: "a" }]);</script>
{#each items as item}
    <input bind:value={item.value}>
{/each}
