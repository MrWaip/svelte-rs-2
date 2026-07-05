App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>x</button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let flag = true;
	const handler_1 = () => {};
	const handler_2 = () => {};
	let handler = $.tag($.derived(() => flag ? handler_1 : handler_2), "handler");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, function(...$$args) {
		$.apply(() => $.get(handler), this, $$args, App, [8, 17]);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
