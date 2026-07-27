App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<!doctype html=""/>`), App[$.FILENAME], [[7, 10]]);
var root_1 = $.add_locations($.from_html(`<button>toggle</button> <!>`, 1), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let show = $.tag($.state(true), "show");
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	{
		var consequent = ($$anchor) => {
			var _doctype = root();
			$.append($$anchor, _doctype);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(show)) $$render(consequent);
		}), "if", App, 7, 0);
	}
	$.delegated("click", button, function click() {
		return $.set(show, !$.get(show));
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
