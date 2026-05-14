<script>
    let rows = [{ name: 'a' }, { name: 'b' }];

    function add() {
        rows = [...rows, { name: 'c' }];
    }
</script>

<button onclick={add}>add</button>

{#each rows as row, idx}
    {@const label = row.name + idx}
    <p>{label}</p>
{/each}
