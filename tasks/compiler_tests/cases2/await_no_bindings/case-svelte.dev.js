App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>Done</p>`), App[$.FILENAME], [[8, 1]]);
var root_1 = $.add_locations($.from_html(`<p>Error</p>`), App[$.FILENAME], [[10, 1]]);
var root_2 = $.add_locations($.from_html(`<p>Loading</p>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const promise = fetch("/api");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.await(node, () => promise, ($$anchor) => {
		var p_2 = root_2();
		$.append($$anchor, p_2);
	}, ($$anchor) => {
		var p = root();
		$.append($$anchor, p);
	}, ($$anchor) => {
		var p_1 = root_1();
		$.append($$anchor, p_1);
	}), "await", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
