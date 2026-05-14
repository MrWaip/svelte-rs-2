<svelte:options runes={false} />
<script lang="ts">
    let action: (node: Node) => void;
    export let x = false;
</script>

<div>
    {#if x}a{/if}
</div>
