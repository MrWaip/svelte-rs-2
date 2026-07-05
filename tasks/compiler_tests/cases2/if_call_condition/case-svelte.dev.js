App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>even</p>`), App[$.FILENAME], [[10, 1]]);
var root_1 = $.add_locations($.from_html(`<p>odd</p>`), App[$.FILENAME], [[12, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 0;
	function is_even() {
		return $.strict_equals(count % 2, 0);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		};
		var d = $.derived(() => is_even());
		var alternate = ($$anchor) => {
			var p_1 = root_1();
			$.append($$anchor, p_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(d)) $$render(consequent);
			else $$render(alternate, -1);
		}), "if", App, 9, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
