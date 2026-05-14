<svelte:options runes={false} />

<script lang="ts">
    type Cb = () => void;
    export let onCb: Cb = (() => {}) as unknown as Cb;
</script>

{onCb}
