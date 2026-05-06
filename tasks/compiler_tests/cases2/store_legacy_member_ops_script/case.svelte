<script>
	import { obj } from './stores';
	function go() {
		$obj.x = 1;
		$obj.x += 1;
		$obj.x ??= 5;
		$obj.x++;
		++$obj.x;
	}
</script>
<button onclick={go}>go</button>
