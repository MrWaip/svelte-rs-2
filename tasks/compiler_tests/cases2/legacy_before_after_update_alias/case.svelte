<script>
	import { beforeUpdate as before, afterUpdate as after } from 'svelte';

	before(() => {
		console.log('before');
	});

	after(() => {
		console.log('after');
	});
</script>

<p>hooks</p>
