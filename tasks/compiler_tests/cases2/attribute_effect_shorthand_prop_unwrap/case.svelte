<script>
    let { width = 1, alt, ...rest } = $props();
</script>

<div {width} {alt} {...rest}></div>
