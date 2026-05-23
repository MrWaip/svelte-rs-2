import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let a = $.mutable_source(null);
	let b = null;
	let c = null;
	function load(source) {
		(($$value) => {
			var $$array = $.to_array($$value, 3);
			$.set(a, $$array[0]);
			b = $$array[1];
			c = $$array[2];
		})(source());
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(a)));
	$.delegated("click", button, () => load(() => [
		1,
		2,
		3
	]));
	$.append($$anchor, button);
}
$.delegate(["click"]);
