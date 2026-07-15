<script>
	import { writable } from 'svelte/store';
	const count = writable(0);

	function foo() {
		return $count;
	}
</script>
