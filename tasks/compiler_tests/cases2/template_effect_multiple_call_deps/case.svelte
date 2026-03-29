<script>
	import { scale } from './utils.js';
	let data = $state([10, 20, 30]);
	const x = $derived(scale([0, data.length], [0, 100]));
	const y = $derived(scale([0, 30], [100, 0]));
</script>

<polyline points={data.map((d, i) => [x(i), y(d)]).join(' ')} />
