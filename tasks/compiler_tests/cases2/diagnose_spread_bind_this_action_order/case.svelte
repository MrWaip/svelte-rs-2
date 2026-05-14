<script>
    import { act } from './act';
    let ref;
    let attrs = {};
</script>

<div {...attrs} use:act bind:this={ref}></div>
