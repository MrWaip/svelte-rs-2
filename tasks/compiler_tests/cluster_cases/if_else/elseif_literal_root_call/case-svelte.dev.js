import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let x = $.prop($$props, "x", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("eee");
			$.append($$anchor, text);
		};
		var consequent_1 = ($$anchor) => {
			var text_1 = $.text("def");
			$.append($$anchor, text_1);
		};
		var alternate = ($$anchor) => {
			var text_2 = $.text("rrr");
			$.append($$anchor, text_2);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.untrack(() => "Eva".startsWith("E"))) $$render(consequent);
			else if (x()) $$render(consequent_1, 1);
			else $$render(alternate, -1);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
