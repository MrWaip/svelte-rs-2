<script>
	import { obj } from './stores';
	let key = 'x';
	function go() {
		$obj["k"] = 1;
		$obj[key] = 2;
		$obj["k"] += 1;
		$obj[key]++;
		$obj["k"] ??= 5;
	}
</script>
<button onclick={go}>go</button>
