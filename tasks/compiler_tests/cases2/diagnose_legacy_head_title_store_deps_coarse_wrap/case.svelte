<script>
    import { writable } from 'svelte/store';
    const toolbar = writable({ title: '' });
    function getTitle(t) { return t; }
</script>

<svelte:head>
    <title>{getTitle($toolbar.title)}</title>
</svelte:head>
