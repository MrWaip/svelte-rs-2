<script>
    export let src;

    const { a, b, c } = src;
    const [d, e] = src.list;
    const { f, g: { h, i: j } } = src.nested;
    const { k: [l, m] } = src.mixed;
    const [n, { o, p }] = src.arrobj;
    const { q: r, s: t } = src.renamed;

    function mutate() {
        b.x = 1;
        e.x = 2;
        h.x = 3;
        l.x = 4;
        o.x = 5;
        r.x = 6;
    }
</script>

<button on:click={mutate}>
    {a} {b.x} {c}
    {d} {e.x}
    {f} {h.x} {j}
    {l.x} {m}
    {n} {o.x} {p}
    {r.x} {t}
</button>
