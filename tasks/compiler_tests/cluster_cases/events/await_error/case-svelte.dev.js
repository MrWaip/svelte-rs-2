App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>x</button>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let promise = Promise.reject();
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.await(node, () => promise, null, void 0, ($$anchor, handler) => {
		var button = root();
		$.delegated("click", button, function(...$$args) {
			$.apply(() => $.get(handler), this, $$args, App, [6, 18]);
		});
		$.append($$anchor, button);
	}), "await", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
