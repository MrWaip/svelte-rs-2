<script>
    import Inner from './Inner.svelte';
    function f() {}
    function g() {}
</script>

<button on:click={f}>
    <Inner onclick={g} />
</button>
