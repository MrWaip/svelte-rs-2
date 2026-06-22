import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let field = $.prop($$props, "field", 8);
	let items = [{}];
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 1, () => items, $.index, ($$anchor, $$item) => {
		let value = () => $.get($$item)[field()];
		var button = root_1();
		var text = $.child(button, true);
		$.reset(button);
		$.template_effect(() => $.set_text(text, value()));
		$.append($$anchor, button);
	});
	$.append($$anchor, fragment);
}
