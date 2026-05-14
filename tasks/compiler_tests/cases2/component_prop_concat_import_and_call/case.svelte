<script>
    import Comp from './Comp.svelte';
    import { BRAND } from './brand';
    import { getName } from './helpers';
</script>

<Comp title="prefix {BRAND}{getName()}" />
