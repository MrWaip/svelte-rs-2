<script>
	import { obj, count } from './stores';
	function go() {
		$count = 1;
		$count++;
		$obj.x = 1;
		$obj.x++;
	}
</script>
<p>{$count}</p>
<p>{$obj.x}</p>
<button onclick={go}>go</button>
