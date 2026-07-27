import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>increment</button> <p> </p> <p> </p> <p> </p> <p> </p>`, 1);
export default function App($$anchor) {
	let count = $.state(1);
	let arr = $.proxy([1, 2]);
	var squared, cubed, toFixed, toString, a, b;
	var $$promises = $.run([async () => {
		var $$d = await $.async_derived(() => ({
			squared: $.get(count) ** 2,
			cubed: $.get(count) ** 3
		}));
		squared = $.derived(() => $.get($$d).squared);
		cubed = $.derived(() => $.get($$d).cubed);
	}, () => {
		toFixed = $.derived(() => $.get(count).toFixed);
		toString = $.derived(() => $.get(count).toString);
		var $$array = $.derived(() => $.to_array(arr, 2));
		a = $.derived(() => $.get($$array)[0]);
		b = $.derived(() => $.get($$array)[1]);
	}]);
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	var text = $.child(p);
	$.reset(p);
	var p_1 = $.sibling(p, 2);
	var text_1 = $.child(p_1);
	$.reset(p_1);
	var p_2 = $.sibling(p_1, 2);
	var text_2 = $.child(p_2);
	$.reset(p_2);
	var p_3 = $.sibling(p_2, 2);
	var text_3 = $.child(p_3);
	$.reset(p_3);
	$.template_effect(() => {
		$.set_text(text, `${$.get(count) ?? ""} ** 2 = ${$.get(squared) ?? ""}`);
		$.set_text(text_1, `${$.get(count) ?? ""} ** 3 = ${$.get(cubed) ?? ""}`);
		$.set_text(text_2, `${typeof $.get(toFixed)} ${typeof $.get(toString)}`);
		$.set_text(text_3, `${$.get(a) ?? ""} ${$.get(b) ?? ""}`);
	}, void 0, void 0, [
		$$promises[1],
		$$promises[0],
		$$promises[1]
	]);
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
