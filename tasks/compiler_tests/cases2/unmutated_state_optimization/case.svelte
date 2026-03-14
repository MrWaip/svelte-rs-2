<script>
    let count = $state(0);
    let label = $state("hello");
    let items = $state([1, 2, 3]);

    function increment() {
        count += 1;
    }
</script>

<button onclick={increment}>{count}</button>
<p>{label}</p>
<ul>
    {#each items as item}
        <li>{item}</li>
    {/each}
</ul>
