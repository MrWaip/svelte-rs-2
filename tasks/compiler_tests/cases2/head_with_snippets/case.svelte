<script>
    let title = $state("hello");
</script>

<svelte:head>
    <meta name="description" content="test">
</svelte:head>

{#snippet badge(text)}
    <span class="badge">{text}</span>
{/snippet}

<div>
    <p>{title}</p>
    {@render badge("new")}
</div>
