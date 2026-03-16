<script>
  let count = $state(0);

  $effect.root(() => {
    console.log("root effect:", count);
  });
</script>

<p>{count}</p>
