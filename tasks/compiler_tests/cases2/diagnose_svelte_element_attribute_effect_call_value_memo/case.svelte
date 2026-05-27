<svelte:options runes={false} />

<script>
    export let href = undefined;

    function getTag() {
        return href ? 'a' : 'div';
    }
    function getRole() {
        return href ? 'link' : undefined;
    }
</script>

<svelte:element this={getTag()} role={getRole()} {href}>x</svelte:element>
