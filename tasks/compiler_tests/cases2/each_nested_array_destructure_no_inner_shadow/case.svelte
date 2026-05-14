<script lang="ts">
    interface Item { name: string; href: string }
    interface Props { data: { schema: string[] } }
    let { data }: Props = $props();
    const groups = $derived.by(() => {
        const groups = new Map<string, Item[]>();
        for (const x of data.schema) groups.set(x, [{ name: x, href: x }]);
        return groups;
    });
</script>

{#each groups as [group, links]}
    <div>
        {group}
        {#each links as { name, href }}
            <a {href}>{name}</a>
        {/each}
    </div>
{/each}
