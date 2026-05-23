<script context="module">
    export const M = 1;
</script>

<script>
    export let a = 0;
    export let b = '';
    $: b = b || (a ? 'x' : 'y');
</script>

<p>{b}</p>
