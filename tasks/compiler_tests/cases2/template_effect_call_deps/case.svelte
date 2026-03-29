<script>
	import { transform } from './utils.js';
	let value = $state(0);
	const fn = $derived(transform(value));
</script>

<div class="output">{fn(value)}</div>
