import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[23, 4]]);
var root_1 = $.add_locations($.from_html(`<!> <button>x</button>`, 1), App[$.FILENAME], [[25, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	function loader() {
		return { data: { selected: null } };
	}
	const state = $.tag($.mutable_source(loader()), "state");
	function reset() {
		if ($.strict_equals($.get(state).data, null)) return;
		$.mutate(state, $.get(state).data.selected = null);
	}
	function pick(value) {
		$.mutate(state, $.get(state).data.selected = value);
	}
	var $$exports = { ...$.legacy_api() };
	var fragment = root_1();
	var node = $.first_child(fragment);
	{
		var consequent = ($$anchor) => {
			var button = root();
			var text = $.child(button, true);
			$.reset(button);
			$.template_effect(() => $.set_text(text, ($.get(state), $.untrack(() => $.get(state).data.selected))));
			$.delegated("click", button, reset);
			$.append($$anchor, button);
		};
		$.add_svelte_meta(() => $.if(node, ($$render) => {
			if ($.get(state), $.untrack(() => $.get(state).data)) $$render(consequent);
		}), "if", App, 22, 0);
	}
	var button_1 = $.sibling(node, 2);
	$.delegated("click", button_1, function click() {
		return pick("x");
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
