<script>
	import { writable } from "svelte/store";
	let { a, c } = { a: 1, c: writable(2) };
	export { a, c };
</script>
<p>{a}{$c}</p>