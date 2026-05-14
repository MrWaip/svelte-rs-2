<script>
    export let source;

    $: ({ users: [{ name: first }, { name: second }], total } = source);
    $: ({ a: [b, { c: [d, e] }] } = source);
</script>

<p>{first}-{second}-{total}-{b}-{d}-{e}</p>
