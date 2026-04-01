<script>
  let count = $state(0);
  count = 1;
</script>

<div>
  before
  {#key count % 2}
    <span>{count}</span>
  {/key}
  after
</div>
