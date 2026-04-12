<script>
	import { beforeUpdate, afterUpdate } from 'svelte';

	beforeUpdate(() => {
		console.log('before');
	});

	afterUpdate(() => {
		console.log('after');
	});
</script>

<p>hooks</p>
