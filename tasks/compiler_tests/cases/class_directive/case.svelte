
<script>
    let absolute = $state();
    let visible = $state();
    let unchanged = $state();
    let untouched = $state();

    const staticClass = true;

    visible = 12;
    absolute = true;
</script>

<div 
    class:visible 
    class:absolute={absolute} 
    class:unchanged 
    class:untouched={untouched}
    class:staticClass
    class:static2={true}
>
    Lorem
</div>