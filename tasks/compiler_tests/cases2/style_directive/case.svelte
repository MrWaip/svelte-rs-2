<script>
    let color = $state('red');
    let fontSize = $state('16px');
    let bg = $state('blue');
    let columns = $state(3);
    const staticVal = 'bold';
</script>

<div
    style:color
    style:--columns={columns}
    style:font-size={fontSize}
    style:background-color={bg}
    style:font-weight={staticVal}
>
    Styled
</div>
