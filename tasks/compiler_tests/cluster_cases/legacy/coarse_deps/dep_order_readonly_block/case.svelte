<script>
	import { writable } from "svelte/store";
	import { foo } from "lib";
	const obj = writable({});
	let c = "";
	$: {
		c;
		foo;
		$obj;
	}
</script>

<input bind:value={c} />
