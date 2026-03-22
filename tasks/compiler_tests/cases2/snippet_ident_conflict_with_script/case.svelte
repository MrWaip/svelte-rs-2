<script>
    function action(node, arg) {
        return { destroy() {} };
    }
</script>

{#snippet card(heading)}
    <div>
        <h3>{heading}</h3>
        {@render badge("new")}
    </div>
{/snippet}
