<script lang="ts">
    let items = $state<number[]>([1, 2, 3]);
</script>

{#each items as item}
    {@const doubled: number = item * 2}
    <p>{doubled}</p>
{/each}
