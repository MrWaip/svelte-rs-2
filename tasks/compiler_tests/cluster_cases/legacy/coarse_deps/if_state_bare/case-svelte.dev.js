import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>+</button><!>`, 1), App[$.FILENAME], [[1, 60]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = $.tag($.mutable_source(1), "foo");
	function inc() {
		$.set(foo, $.get(foo) + 1);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var button = $.first_child(fragment);
	var node = $.sibling(button);
	{
		var consequent = ($$anchor) => {
			var text = $.text("a");
			$.append($$anchor, text);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(foo)) $$render(consequent);
		}), "if", App, 1, 93);
	}
	$.event("click", button, inc);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
