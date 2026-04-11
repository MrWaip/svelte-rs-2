<script>
    import Widget from './Widget.svelte';
</script>

<Widget>
    {@const foo = 1}
    {#snippet children()}
        <p>{foo}</p>
    {/snippet}
</Widget>
