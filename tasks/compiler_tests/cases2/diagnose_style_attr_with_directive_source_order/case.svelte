<script>
    let { style = '', size = 's', label = '' } = $props();
</script>

<div
    {style}
    class="box {size}"
    style:width="100px"
    data-label={label}
></div>
