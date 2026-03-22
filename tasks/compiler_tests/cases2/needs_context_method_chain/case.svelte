<script>
	import { createFormatter } from './utils.js';

	let { data } = $props();

	let items = $state([]);

	// member on call result: items.filter(Boolean).length
	let total = $derived(items.filter(Boolean).length);

	// call on import: createFormatter()
	let fmt = createFormatter();

	// member on import: createFormatter.defaults
	let defaults = createFormatter.defaults;

	// member on prop: data.nested
	let nested = data.nested;

	// new expression
	let map = new Map();
</script>

<p>{total} {fmt} {defaults} {nested} {map}</p>
