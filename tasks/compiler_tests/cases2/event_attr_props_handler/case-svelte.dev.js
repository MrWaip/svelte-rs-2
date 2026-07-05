App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>x</button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, function(...$$args) {
		$.apply(() => $$props.onChange, this, $$args, App, [5, 17]);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
