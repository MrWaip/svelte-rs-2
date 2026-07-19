<script>
	import { noop } from './x.js';
	;
	let count = 0;
	noop(count);
</script>

<button onclick={() => count++}>{count}</button>
