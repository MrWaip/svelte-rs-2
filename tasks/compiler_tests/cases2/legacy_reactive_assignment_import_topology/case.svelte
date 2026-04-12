<script>
	import data from './dep.js';

	function bump() {
		data.count += 1;
	}

	$: doubled = total * 2;
	$: total = data.count;
</script>

<button onclick={bump}>{doubled}</button>
