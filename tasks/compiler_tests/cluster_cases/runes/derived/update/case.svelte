<script>
	let count = $derived(0);
	let postfix = count++;
	let postfix_minus = count--;
	let prefix = ++count;
	let prefix_minus = --count;
</script>

<p>{postfix}, {postfix_minus}, {prefix}, {prefix_minus}, {count}</p>
