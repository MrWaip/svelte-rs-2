<script>
	import { writable } from 'svelte/store';

	const client = writable({ bankEmail: '' });

	function onSuccess(email) {
		client.update(($client) => {
			$client.bankEmail = email;
			return $client;
		});
	}
</script>

<button onclick={() => onSuccess('x')}>go</button>
