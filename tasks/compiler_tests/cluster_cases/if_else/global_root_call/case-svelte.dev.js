import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("eee");
			$.append($$anchor, text);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.untrack(() => Math.max(1, 2) > 1)) $$render(consequent);
		}), "if", App, 1, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
