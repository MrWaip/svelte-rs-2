<script>
    export let show;
    export let value;
</script>

<li>
    <slot name="item">
        {#if show}
            <span>{value}</span>
        {/if}
        <div>tail</div>
    </slot>
</li>
