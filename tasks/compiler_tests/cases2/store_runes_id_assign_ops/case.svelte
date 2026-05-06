<script>
	import { count } from './stores';
	function go() {
		$count = 1;
		$count += 1;
		$count -= 1;
		$count *= 2;
		$count /= 2;
		$count %= 3;
		$count **= 2;
		$count &&= 5;
		$count ||= 5;
		$count ??= 5;
		$count &= 1;
		$count |= 1;
		$count ^= 1;
		$count <<= 1;
		$count >>= 1;
		$count >>>= 1;
		$count++;
		$count--;
		++$count;
		--$count;
	}
</script>
<button onclick={go}>go</button>
