App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>go</button>`), App[$.FILENAME], [[13, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let cache = $.tag_proxy($.proxy({}), "cache");
	async function go() {
		const value = (await $.track_reactivity_loss($.assign_async(cache, "value", "??=", () => get_value(), "(unknown):5:16")))();
	}
	async function get_value() {
		return 42;
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, go);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
