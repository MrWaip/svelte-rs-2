App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/server";
import { writable } from "svelte/store";
function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const client = writable({ bankEmail: "" });
		function onSuccess(email) {
			client.update(($client) => {
				$.store_mutate($$store_subs ??= {}, "$client", client, $client.bankEmail = email);
				return $client;
			});
		}
		$$renderer.push(`<button>`);
		$.push_element($$renderer, "button", 14, 0);
		$$renderer.push(`go</button>`);
		$.pop_element();
	}, App);
}
App.render = function() {
	throw new Error("Component.render(...) is no longer valid in Svelte 5. See https://svelte.dev/docs/svelte/v5-migration-guide#Components-are-no-longer-classes for more information");
};
export default App;
