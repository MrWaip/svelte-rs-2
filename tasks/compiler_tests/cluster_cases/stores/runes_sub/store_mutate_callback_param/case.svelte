<script>
	import { writable } from "svelte/store";
	const client = writable({});
	function onOtp(email) {
		client.update(($client) => {
			if ($client) {
				$client.bankEmail = email;
			}
			return $client;
		});
	}
</script>

<button onclick={() => onOtp("a@b.c")}>set</button>
