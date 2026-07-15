App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { writable } from "svelte/store";
var root = $.add_locations($.from_html(`<button>go</button>`), App[$.FILENAME], [[14, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const client = writable({ bankEmail: "" });
	function onSuccess(email) {
		client.update(($client) => {
			$client.bankEmail = email;
			return $client;
		});
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, function click() {
		return onSuccess("x");
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
