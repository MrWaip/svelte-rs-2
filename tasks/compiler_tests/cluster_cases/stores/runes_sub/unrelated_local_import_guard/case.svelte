<script>
	import { helper } from './store.js';
	let a = $state(0);
</script>

<button onclick={() => a++}>{a} {helper}</button>
