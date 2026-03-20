<script lang="ts">
    type Handler = (e: MouseEvent) => void;
    let handler: Handler = $state(() => {});
</script>

<button onclick={handler as any}>click</button>
