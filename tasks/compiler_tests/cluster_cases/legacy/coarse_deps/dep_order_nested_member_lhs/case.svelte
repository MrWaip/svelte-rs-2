<script>
	import { writable } from "svelte/store";
	import { foo } from "lib";
	const obj = writable({});
	let c = "";
	$: $obj.a.b = (c ? c : "") + foo($obj.x);
</script>

<input bind:value={c} />
