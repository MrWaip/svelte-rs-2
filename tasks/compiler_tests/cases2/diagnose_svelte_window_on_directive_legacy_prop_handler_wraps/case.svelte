<script>
    export let closeModal;
</script>

<svelte:window on:close|preventDefault={closeModal} />
