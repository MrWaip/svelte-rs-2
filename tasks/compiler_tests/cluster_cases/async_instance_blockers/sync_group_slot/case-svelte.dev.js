import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>inc</button> <p> </p> <p> </p> <p> </p> <p> </p>`, 1), App[$.FILENAME], [
	[10, 0],
	[11, 0],
	[12, 0],
	[13, 0],
	[14, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let gate = $.tag($.state(0), "gate");
	var one, sync1, sync2, two, sync3;
	var $$promises = $.run([
		async () => one = await $.async_derived(async () => (await $.track_reactivity_loss($.get(gate)))(), "one", "(unknown):3:11"),
		() => {
			sync1 = $.get(gate) + 1;
			sync2 = $.get(gate) + 2;
		},
		async () => two = await $.async_derived(async () => (await $.track_reactivity_loss($.get(gate)))(), "two", "(unknown):6:11"),
		() => sync3 = $.get(gate) + 3
	]);
	var $$exports = { ...$.legacy_api() };
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
	$.delegated("click", button, function click() {
		return $.update(gate);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
