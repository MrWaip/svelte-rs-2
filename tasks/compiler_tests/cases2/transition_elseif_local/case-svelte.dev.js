App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { fade } from "svelte/transition";
var root = $.add_locations($.from_html(`<p>first</p>`), App[$.FILENAME], [[9, 1]]);
var root_1 = $.add_locations($.from_html(`<div>second</div>`), App[$.FILENAME], [[11, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = false;
	let y = true;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var p = root();
			$.append($$anchor, p);
		};
		var consequent_1 = ($$anchor) => {
			var div = root_1();
			$.transition(3, div, () => fade);
			$.append($$anchor, div);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (x) $$render(consequent);
			else if (y) $$render(consequent_1, 1);
		}), "if", App, 8, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
