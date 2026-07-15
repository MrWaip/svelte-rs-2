import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.from_html(`<button>set</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	const client = writable({});
	function onOtp(email) {
		client.update(($client) => {
			if ($client) {
				$client.bankEmail = email;
			}
			return $client;
		});
	}
	var button = root();
	$.delegated("click", button, () => onOtp("a@b.c"));
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
