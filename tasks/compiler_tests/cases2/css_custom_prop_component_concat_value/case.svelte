<script>
  import Child from './Child.svelte';
  let val = $state('25');
</script>

<div>
  <Child --color="px {val}" />
</div>
<button onclick={() => (val = '50')}>x</button>
