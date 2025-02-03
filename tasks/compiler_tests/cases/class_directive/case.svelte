
<script>
    let absolute = $state();
    let visible = $state();
    let unchanged = $state();
    let untouched = $state();

    visible = 12;
    absolute = true;
</script>

<div class:visible class:absolute={absolute} class:unchanged class:untouched={untouched}>
    Lorem
</div>