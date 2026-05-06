<script>
    let obj = { x: 0 };
    function bump() { obj.x += 1; }
</script>
<button onclick={bump}>{obj.x + 1}</button>
