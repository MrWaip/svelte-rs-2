<script>
import { writable } from 'svelte/store';
let state = writable(0);
let x = $state(0);
</script>
