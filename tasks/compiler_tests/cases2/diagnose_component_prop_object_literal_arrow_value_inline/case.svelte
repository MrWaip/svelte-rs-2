<script>
  import Child from './Child.svelte';
  import { invalidate } from './lib';
</script>

<Child props={{ onStart: () => invalidate(true) }} />
