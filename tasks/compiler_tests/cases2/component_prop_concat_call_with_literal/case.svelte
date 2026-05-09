<script>
    import Comp from './Comp.svelte';
    import { getProductName } from './helpers';
</script>

<Comp title="x{0}{getProductName()}" />
