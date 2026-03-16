<script>
  let count = $state(0);

  const cleanup = $effect.root(() => {
    console.log("root effect:", count);
    return () => console.log("cleanup");
  });
</script>

<p>{count}</p>
