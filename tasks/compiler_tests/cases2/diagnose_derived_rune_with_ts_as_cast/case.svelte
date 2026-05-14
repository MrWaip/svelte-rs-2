<script lang="ts">
    let { error, fallback }: { error?: string; fallback: string } = $props();
    let status = $derived(error ? 'error' : fallback) as string;
</script>

<span>{status}</span>
