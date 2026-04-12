<script>
    import Widget from "./Widget.svelte";
</script>

<style>
    .host .inside { color: red; }
</style>

<div class="host">
    <Widget>
        {#snippet children()}
            <span class="inside">inside</span>
        {/snippet}
    </Widget>
</div>

<span class="inside">outside</span>
