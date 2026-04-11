<script module>
    const greeting = () => "module fn";
    export { greeting };
</script>

{#snippet greeting()}
    <p>template snippet</p>
{/snippet}
