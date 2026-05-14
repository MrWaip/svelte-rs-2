<svelte:options runes={false} />
<script>
    export let challenge;
    export let error;

    $: phone = $challenge?.phone || '';
</script>

<input bind:value={$error} />
<p>{phone}</p>
