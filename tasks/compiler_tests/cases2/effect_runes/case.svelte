<script>
  let count = $state(0);
  count += 1;

  $effect(() => {
    console.log("count:", count);
  });

  $effect.pre(() => {
    console.log("pre-effect:", count);
  });
</script>
<p>{count}</p>
