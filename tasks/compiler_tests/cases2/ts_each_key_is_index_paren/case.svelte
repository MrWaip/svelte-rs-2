<script lang="ts">
    let items = $state(['a','b','c']);
</script>
{#each items as item, i ((i))}
    <p>{i}: {item}</p>
{/each}
