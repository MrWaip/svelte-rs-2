<script>
  import { slide } from 'svelte/transition';
  export let rest = {};
</script>
<div {...rest} transition:slide></div>
