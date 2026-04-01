import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let answer = $.prop($$props, "answer", 7), rest = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		"$$host",
		"answer"
	]);
	let host = $$props.$$host;
	var $$exports = {
		get answer() {
			return answer();
		},
		set answer($$value) {
			answer($$value);
			$.flush();
		}
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, answer()));
	$.append($$anchor, p);
	return $.pop($$exports);
}
customElements.define("my-element", $.create_custom_element(App, { answer: {} }, [], [], { mode: "open" }));
