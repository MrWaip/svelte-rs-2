import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> <p> </p> <p> </p> <p> </p> <p> </p>`, 1);
export default function App($$anchor) {
	let gate = $.state(0);
	var one, sync1, sync2, two, sync3;
	var $$promises = $.run([
		async () => one = await $.async_derived(() => $.get(gate)),
		() => {
			sync1 = $.get(gate) + 1;
			sync2 = $.get(gate) + 2;
		},
		async () => two = await $.async_derived(() => $.get(gate)),
		() => sync3 = $.get(gate) + 3
	]);
	var fragment = root();
	var button = $.first_child(fragment);
	var p = $.sibling(button, 2);
	var text = $.child(p, true);
	$.reset(p);
	var p_1 = $.sibling(p, 2);
	var text_1 = $.child(p_1);
	$.reset(p_1);
	var p_2 = $.sibling(p_1, 2);
	var text_2 = $.child(p_2, true);
	$.reset(p_2);
	var p_3 = $.sibling(p_2, 2);
	var text_3 = $.child(p_3, true);
	$.reset(p_3);
	$.template_effect(() => {
		$.set_text(text, $.get(one));
		$.set_text(text_1, `${sync1}${sync2}`);
		$.set_text(text_2, $.get(two));
		$.set_text(text_3, sync3);
	}, void 0, void 0, [
		$$promises[0],
		$$promises[1],
		$$promises[1],
		$$promises[2],
		$$promises[3]
	]);
	$.delegated("click", button, () => $.update(gate));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
