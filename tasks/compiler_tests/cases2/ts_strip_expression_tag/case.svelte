<script lang="ts">
    let value = $state<string>('hello');
</script>

<p>{value as string}</p>
