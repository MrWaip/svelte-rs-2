App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>Click</button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(0), "count");
	$.user_effect(() => {
		let double = $.tag($.derived(() => $.get(count) * 2), "double");
		console.log($.get(double));
	});
	var $$exports = { ...$.legacy_api() };
	var button = root();
	$.delegated("click", button, function click() {
		return $.update(count);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
