<script>
	import { api } from './api.js';

	function doSomething() {
		api.call();
	}
</script>

<button onclick={doSomething}>click</button>
