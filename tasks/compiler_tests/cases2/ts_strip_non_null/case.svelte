<script lang="ts">
    let value = $state<string | null>('hello');
</script>

<p>{value!}</p>
