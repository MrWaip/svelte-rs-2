<script>
    import Widget from './Widget.svelte';
</script>

<Widget>
    {#snippet children()}
        {@const foo = 1}
        <p>{foo}</p>
    {/snippet}
</Widget>
