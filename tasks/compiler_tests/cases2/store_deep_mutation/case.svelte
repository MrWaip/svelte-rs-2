<script>
	import { store } from './stores';
	$store.field = 'hello';
</script>

<button onclick={() => $store.field = 'world'}>set</button>
