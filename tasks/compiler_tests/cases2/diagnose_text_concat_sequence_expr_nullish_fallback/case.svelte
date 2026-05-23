<script>
    import { Kind } from './kinds';
    export let item;
</script>

<span>Prefix {item?.kind === Kind.A ? 'one' : 'two'} suffix</span>
