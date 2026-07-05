import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[13, 4]]);
var root_1 = $.add_locations($.from_html(`<button>+</button> <!>`, 1), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let count = $.tag($.mutable_source(1), "count");
	function inc() {
		$.set(count, $.get(count) + 1);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	{
		var consequent = ($$anchor) => {
			const label = $.tag($.derived_safe_equal(() => ($.get(count), $.untrack(() => $.get(count).toFixed(2)))), "label");
			$.get(label);
			var span = root();
			var text = $.child(span, true);
			$.reset(span);
			$.template_effect(() => $.set_text(text, $.get(label)));
			$.append($$anchor, span);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(count)) $$render(consequent);
		}), "if", App, 11, 0);
	}
	$.event("click", button, inc);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
