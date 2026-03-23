<script>
import Input from './Input.svelte';
let { value = $bindable() } = $props();
</script>
<Input bind:value />
