<script>
    export let x;
</script>

<div class:before-content={$$slots.beforeContent}>
    <slot name="beforeContent" />
    {x}
</div>
