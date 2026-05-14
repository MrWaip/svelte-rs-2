<script>
    let { wrapWith = defaultWrapWith, label = '' } = $props();
    let count = $state(0);
</script>

{#snippet defaultWrapWith(mf)}
    {@render mf()}
{/snippet}

{#snippet inner()}
    <span>{label}{count}</span>
{/snippet}

<div>{@render wrapWith(inner)}</div>

<svelte:head>
    <style>
        :root { --x: red; }
    </style>
</svelte:head>
