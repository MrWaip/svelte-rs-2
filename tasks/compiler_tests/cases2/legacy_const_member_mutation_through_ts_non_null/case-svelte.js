import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
var root_1 = $.from_html(`<!> <button>x</button>`, 1);
export default function App($$anchor) {
	function loader() {
		return { data: { selected: null } };
	}
	const state = $.mutable_source(loader());
	function reset() {
		if ($.get(state).data === null) return;
		$.mutate(state, $.get(state).data.selected = null);
	}
	function pick(value) {
		$.mutate(state, $.get(state).data.selected = value);
	}
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
		$.if(node, ($$render) => {
			if ($.get(state), $.untrack(() => $.get(state).data)) $$render(consequent);
		});
	}
	var button_1 = $.sibling(node, 2);
	$.delegated("click", button_1, () => pick("x"));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
