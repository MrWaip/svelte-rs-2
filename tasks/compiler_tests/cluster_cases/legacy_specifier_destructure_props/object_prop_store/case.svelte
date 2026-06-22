<script>
	import { writable } from "svelte/store";
	let { a, c } = { a: 1, c: writable(2) };
	export { a, c };
	function inc() { a++; }
</script>
<button on:click={inc}>{a}{$c}</button>