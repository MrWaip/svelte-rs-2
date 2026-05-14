<script>
let { foo } = $props();
</script>

{#if foo.bar === 'x'}a{:else}b{/if}
