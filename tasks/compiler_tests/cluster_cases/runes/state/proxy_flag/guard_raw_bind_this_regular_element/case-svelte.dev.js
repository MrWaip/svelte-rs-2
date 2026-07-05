App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let elem = $.tag($.state(void 0), "elem");
	$.user_effect(() => {
		console.log(...$.log_if_contains_state("log", $.get(elem)));
	});
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.bind_this(div, ($$value) => $.set(elem, $$value), () => $.get(elem));
	$.append($$anchor, div);
	return $.pop($$exports);
}
