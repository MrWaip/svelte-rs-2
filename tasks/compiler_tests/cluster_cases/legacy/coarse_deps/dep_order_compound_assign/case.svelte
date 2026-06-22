<script>
	import { foo } from "lib";
	let c = 0;
	$: c += foo();
</script>

<input bind:value={c} />
