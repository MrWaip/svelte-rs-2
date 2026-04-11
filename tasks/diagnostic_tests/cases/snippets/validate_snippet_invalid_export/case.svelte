<script module>
    export { greeting };
</script>

<script>
    let message = 'hello';
</script>

{#snippet greeting(name)}
    <p>{message} {name}!</p>
{/snippet}
