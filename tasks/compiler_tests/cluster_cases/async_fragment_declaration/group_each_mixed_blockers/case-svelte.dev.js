import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 26]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var a;
	var $$promises = $.run([async () => a = (await $.track_reactivity_loss(Promise.resolve([1])))()]);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, {}, ($$anchor) => {
		let b;
		var promises = $.run([async () => b = $.tag((await $.save($.async_derived(async () => (await $.save(Promise.resolve([2])))())))(), "b")]);
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.async(node_1, [promises[0], $$promises[0]], void 0, (node_1) => {
			$.add_svelte_meta(() => $.each(node_1, 17, () => [...$.get(b), ...a], $.index, ($$anchor, x) => {
				var p = root();
				var text = $.child(p, true);
				$.reset(p);
				$.template_effect(() => $.set_text(text, $.get(x)));
				$.append($$anchor, p);
			}), "each", App, 6, 1);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
