import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[3, 10]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	function fade(node) {
		return {};
	}
	let show = true;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node_1 = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var div = root();
			$.transition(3, div, () => fade, () => ({ duration: 200 }));
			$.append($$anchor, div);
		};
		$.add_svelte_meta(() => $.if(node_1, ($$render) => {
			if (show) $$render(consequent);
		}), "if", App, 3, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
