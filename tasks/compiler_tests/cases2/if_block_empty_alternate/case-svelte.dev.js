App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>hello</p>`), App[$.FILENAME], [[4, 10]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let show = true;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		};
		var alternate = ($$anchor) => {};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (show) $$render(consequent);
			else $$render(alternate, -1);
		}), "if", App, 4, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
