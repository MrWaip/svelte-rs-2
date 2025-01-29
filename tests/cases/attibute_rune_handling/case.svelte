<script>
    let title = $state(10);
    let name = $state("");

    title = 12;

</script>
<br/>
<input 
    {name}
    name2={name} 
    {title}
    title2={title}
    complex="_{title}+{name}_"
 />