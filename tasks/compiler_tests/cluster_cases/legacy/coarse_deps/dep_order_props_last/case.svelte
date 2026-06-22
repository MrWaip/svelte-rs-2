<script>
  export let id;
  export let callback;

  $: callback(id), $$props;
</script>
