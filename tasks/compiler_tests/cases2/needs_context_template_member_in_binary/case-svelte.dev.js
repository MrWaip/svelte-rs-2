App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var text = $.text("a");
			$.append($$anchor, text);
		};
		var alternate = ($$anchor) => {
			var text_1 = $.text("b");
			$.append($$anchor, text_1);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.strict_equals($$props.foo.bar, "x")) $$render(consequent);
			else $$render(alternate, -1);
		}), "if", App, 5, 0);
	}
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
