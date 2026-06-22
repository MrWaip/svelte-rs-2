<script>
	import { foo } from "lib";
	let obj = {};
	let c = "";
	$: obj.purpose = (c ? c : "") + foo(obj.type);
</script>

<input bind:value={c} />
<input bind:value={obj.x} />
