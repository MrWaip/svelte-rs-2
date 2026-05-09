<script lang="ts">
    import { Kind } from './kinds';

    type Item = { kind: number };
    type Props = { item: Item };

    const { item }: Props = $props();
</script>

{#if item.kind === Kind.A}
    <span>A</span>
{:else}
    <span>B</span>
{/if}
