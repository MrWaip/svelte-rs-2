import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let arr = $.tag_proxy($.proxy([
		1,
		2,
		3
	]), "arr");
	var a, rest, x, others;
	var $$promises = $.run([async () => void (await $.track_reactivity_loss(Promise.resolve()))(), () => {
		var $$array = $.tag($.derived(() => $.to_array(arr)), "[$derived iterable]");
		a = $.tag($.derived(() => $.get($$array)[0]), "a");
		rest = $.tag($.derived(() => $.get($$array).slice(1)), "rest");
		var $$d = $.derived(() => ({ x: 2 }));
		x = $.tag($.derived(() => $.fallback($.get($$d).x, 1)), "x");
		others = $.tag($.derived(() => $.exclude_from_object($.get($$d), ["x"])), "others");
	}]);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""} ${$.get(rest).length ?? ""} ${$.get(x) ?? ""} ${$.get(others).y ?? ""}`), void 0, void 0, [$$promises[1], $$promises[1]]);
	$.append($$anchor, p);
	return $.pop($$exports);
}
