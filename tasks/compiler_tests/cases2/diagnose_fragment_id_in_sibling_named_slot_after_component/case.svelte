<script>
    import Outer from './Outer.svelte';
    import Img from './Img.svelte';
    import Btn from './Btn.svelte';
    let cond = true;
</script>

<Outer>
    <Img slot="image" />
    <Btn slot="action">
        {#if cond}<span>a</span>{:else}b{/if}
    </Btn>
</Outer>
