App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { slide } from "svelte/transition";
var root = $.add_locations($.from_html(`<div>hi</div>`), App[$.FILENAME], [[9, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let visible = true;
	function k() {}
	function c() {}
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var div = root();
			$.delegated("keydown", div, k);
			$.delegated("click", div, c);
			$.transition(3, div, () => slide);
			$.append($$anchor, div);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if (visible) $$render(consequent);
		}), "if", App, 8, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["keydown", "click"]);
