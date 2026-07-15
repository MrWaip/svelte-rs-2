<script>
	import { writable } from 'svelte/store';
	const count = writable(0);

	function foo() {
		let count = 0;
		return $count;
	}
</script>
