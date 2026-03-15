<script>
    let color = $state('red');
    let fontSize = $state('16px');
    let bg = $state('blue');
    const staticVal = 'bold';
</script>

<div
    style:color
    style:font-size={fontSize}
    style:background-color={bg}
    style:font-weight={staticVal}
>
    Styled
</div>
