<script>
	import { writable } from "svelte/store";

	const count = writable(0);
</script>

<div title={[() => $count]}></div>
