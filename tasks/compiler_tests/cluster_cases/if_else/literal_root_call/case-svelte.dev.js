import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("eee");
			$.append($$anchor, text);
		};
		var alternate = ($$anchor) => {
			var text_1 = $.text("rrr");
			$.append($$anchor, text_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.untrack(() => "Eva".startsWith("E"))) $$render(consequent);
			else $$render(alternate, -1);
		}), "if", App, 1, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
