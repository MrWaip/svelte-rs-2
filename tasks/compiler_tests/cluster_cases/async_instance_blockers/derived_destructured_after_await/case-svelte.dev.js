import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button>increment</button> <p> </p> <p> </p> <p> </p> <p> </p>`, 1), App[$.FILENAME], [
	[17, 0],
	[19, 0],
	[20, 0],
	[21, 0],
	[22, 0]
]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.tag($.state(1), "count");
	let arr = $.tag_proxy($.proxy([1, 2]), "arr");
	var squared, cubed, toFixed, toString, a, b;
	var $$promises = $.run([async () => {
		var $$d = await $.async_derived(async () => (await $.track_reactivity_loss({
			squared: $.get(count) ** 2,
			cubed: $.get(count) ** 3
		}))(), "[$derived object]", "(unknown):6:26");
		squared = $.tag($.derived(() => $.get($$d).squared), "squared");
		cubed = $.tag($.derived(() => $.get($$d).cubed), "cubed");
	}, () => {
		toFixed = $.tag($.derived(() => $.get(count).toFixed), "toFixed");
		toString = $.tag($.derived(() => $.get(count).toString), "toString");
		var $$array = $.tag($.derived(() => $.to_array(arr, 2)), "[$derived iterable]");
		a = $.tag($.derived(() => $.get($$array)[0]), "a");
		b = $.tag($.derived(() => $.get($$array)[1]), "b");
	}]);
	var $$exports = { ...$.legacy_api() };
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
	$.delegated("click", button, function click() {
		return $.update(count);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
$.delegate(["click"]);
