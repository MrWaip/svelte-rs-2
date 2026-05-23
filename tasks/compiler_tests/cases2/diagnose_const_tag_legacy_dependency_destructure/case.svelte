<script>
    export let items;
</script>

{#each items as item}
    {@const { button } = item}
    <a href={button.href}>{button.text}</a>
{/each}
