<script>
    import { page } from '$app/stores';
    let { wrapWith = defaultWrapWith } = $props();
</script>

{#snippet defaultWrapWith(mf)}
    {@render mf()}
{/snippet}

{#snippet mf()}
    {#if $page.url}
        <b>x</b>
    {/if}
{/snippet}

<div>{@render wrapWith(mf)}</div>
