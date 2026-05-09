<script>
import { Foo } from './x.js';
let { value } = $props();
</script>

{#if value === Foo.X}a{:else}b{/if}
