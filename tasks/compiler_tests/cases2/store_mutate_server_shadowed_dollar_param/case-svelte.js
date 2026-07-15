import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const client = writable({ bankEmail: "" });
	function onSuccess(email) {
		client.update(($client) => {
			$client.bankEmail = email;
			return $client;
		});
	}
	var button = root();
	$.delegated("click", button, () => onSuccess("x"));
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
