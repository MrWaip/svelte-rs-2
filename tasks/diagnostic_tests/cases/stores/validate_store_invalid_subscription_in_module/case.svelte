<script>
import { writable } from 'svelte/store';
const count = writable(0);
</script>
<script module>
console.log($count);
</script>
