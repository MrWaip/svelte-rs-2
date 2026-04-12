<script>
    let active = $state(true);
</script>

<style>
    .chunk-shell.state .summary {
        color: red;
    }
</style>

{#snippet summary()}
    <section class="summary">summary</section>
{/snippet}

<div class="chunk-shell" class:state={active}>
    {@render summary()}
</div>
