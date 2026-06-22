<script>
	import { numbers } from './data.js';

	function add() {
		numbers[numbers.length] = numbers.length + 1;
	}
</script>

<p>{numbers.join(' + ')}</p>
<button on:click={add}>add</button>
