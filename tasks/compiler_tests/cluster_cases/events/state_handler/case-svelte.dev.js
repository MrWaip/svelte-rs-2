App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>x</button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let handler = $.tag($.state(() => {}), "handler");
	$.user_effect(() => {
		$.set(handler, () => console.log("x"));
	});
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, function(...$$args) {
		$.apply(() => $.get(handler), this, $$args, App, [8, 17]);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
