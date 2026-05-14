<script>
    let { Layout, Btn, Cap } = $props();
</script>

<div>
    <Layout>
        {#snippet footer()}
            <Btn />
            <div class="cap"><Cap /></div>
        {/snippet}
    </Layout>
</div>
