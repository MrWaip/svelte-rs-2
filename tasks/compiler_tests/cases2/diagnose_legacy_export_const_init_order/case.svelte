<svelte:options runes={false} />
<script>
    import { setContext } from 'svelte';

    export let a;
    let local = 0;
    export const getLocal = () => local;

    setContext('k', a);
    local = a;
</script>
