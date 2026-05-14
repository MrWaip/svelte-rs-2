<script>
	import { writable } from "svelte/store";

	const search = writable({ value: "" });
	const error = writable("");
</script>

<slot
	value={$search.value}
	isInvalid={Boolean($error)}
/>
