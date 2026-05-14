<script>
    import Badge from './Badge.svelte';
    export let x;
    function f(n) { return n; }
</script>

<Badge text="a {f(x)} b" />
