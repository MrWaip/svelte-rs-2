<script>
let ok = true;
let node = null;
node = document.body;
</script>

{#if ok}
    <div bind:this={node}></div>
{/if}
