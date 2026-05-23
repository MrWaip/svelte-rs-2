<script lang="ts">
    let items = $state(['a','b']);
</script>
{#each items as item (item as string)}
    <p>{item}</p>
{/each}
