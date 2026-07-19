import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let count = $.state(0);
	function action(node) {
		return { update(count) {
			console.log("update", this.count, this.count = count);
		} };
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.action(button, ($$node) => action?.($$node));
	$.template_effect(() => $.set_text(text, $.get(count)));
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
