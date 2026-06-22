<script>
	import { foo } from "lib";
	let total = 0;
	let c = 0;
	let d = 0;
	$: total = c + foo(d);
</script>

<input bind:value={c} />
<input bind:value={d} />
