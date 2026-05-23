<script>
    import Child from './Child.svelte';
    import { thing } from './lib';
    export let p;
    $: m = p;
</script>

<Child config={{ a: m, b: thing }} --color="red" />
