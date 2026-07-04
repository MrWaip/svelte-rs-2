import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[13, 4]]);
var root_1 = $.add_locations($.from_html(`<button>bump</button> <!>`, 1), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let state = $.tag($.mutable_source(""), "state");
	function bump() {
		$.set(state, $.get(state) + "x");
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var button = $.first_child(fragment);
	var node = $.sibling(button, 2);
	{
		var consequent = ($$anchor) => {
			const localLen = $.tag($.derived_safe_equal(() => ($.get(state), $.untrack(() => $.get(state).length))), "localLen");
			$.get(localLen);
			var span = root();
			var text = $.child(span);
			$.reset(span);
			$.template_effect(() => $.set_text(text, `Length: ${$.get(localLen) ?? ""}`));
			$.append($$anchor, span);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(state)) $$render(consequent);
		}), "if", App, 11, 0);
	}
	$.delegated("click", button, bump);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
