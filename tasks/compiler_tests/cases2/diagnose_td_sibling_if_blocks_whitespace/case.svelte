<script>
    let { a, b } = $props();
</script>

<table>
    <tbody>
        <tr>
            <td>
                {#if a}
                    {a} <br />
                {/if}
                {#if b}
                    {b} <br />
                {/if}
            </td>
        </tr>
    </tbody>
</table>
