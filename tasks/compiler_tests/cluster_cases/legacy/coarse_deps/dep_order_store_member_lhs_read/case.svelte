<script>
	import { writable } from "svelte/store";
	import { foo } from "lib";
	const obj = writable({});
	let c = "";
	$: $obj.purpose = (c ? c : "") + foo({ type: $obj.type });
</script>

<input bind:value={c} />
