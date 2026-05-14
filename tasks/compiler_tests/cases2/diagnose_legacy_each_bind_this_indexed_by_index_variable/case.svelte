<script>
    export let items;
    let refs = [];
</script>

{#each items as item, idx}
    <input bind:this={refs[idx]} />
{/each}
