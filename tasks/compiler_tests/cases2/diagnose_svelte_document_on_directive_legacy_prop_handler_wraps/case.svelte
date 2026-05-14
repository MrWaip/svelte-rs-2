<script>
    export let closeModal;
</script>

<svelte:document on:visibilitychange|preventDefault={closeModal} />
