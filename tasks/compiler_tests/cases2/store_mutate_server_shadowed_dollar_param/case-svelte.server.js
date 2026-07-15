import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const client = writable({ bankEmail: "" });
		function onSuccess(email) {
			client.update(($client) => {
				$.store_mutate($$store_subs ??= {}, "$client", client, $client.bankEmail = email);
				return $client;
			});
		}
		$$renderer.push(`<button>go</button>`);
	});
}
