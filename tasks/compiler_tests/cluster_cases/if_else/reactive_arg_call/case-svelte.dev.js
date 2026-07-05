import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let name = $.prop($$props, "name", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("eee");
			$.append($$anchor, text);
		};
		var d = $.derived(() => ($.deep_read_state(name()), $.untrack(() => "abc".startsWith(name()))));
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(d)) $$render(consequent);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
