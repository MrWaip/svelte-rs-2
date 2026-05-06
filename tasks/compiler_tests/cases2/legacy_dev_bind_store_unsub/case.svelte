<script>
	import { writable } from 'svelte/store';
	let s = writable(0);
	function swap() {
		s = writable(1);
	}
</script>
<input bind:value={$s}>
<button onclick={swap}>swap</button>
