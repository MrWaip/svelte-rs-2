<script>
	import { writable } from "svelte/store";
	let store = $state();
	function setStore() {
		store = writable(0);
	}
</script>

<button onclick={setStore}>set new store</button>
<button onclick={() => $store++}>incr</button>
<pre>{$store}</pre>
