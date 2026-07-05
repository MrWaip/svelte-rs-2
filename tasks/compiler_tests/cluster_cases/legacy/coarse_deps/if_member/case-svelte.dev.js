import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = $.prop($$props, "foo", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("a");
			$.append($$anchor, text);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.deep_read_state(foo()), $.untrack(() => foo().bar)) $$render(consequent);
		}), "if", App, 1, 32);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
