<script>
    import Comp from './Comp.svelte';
    export let toggle = false;
</script>

<Comp description="prefix {toggle ? 'A' : 'B'}" />
