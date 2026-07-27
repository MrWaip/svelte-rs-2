import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p>yes</p>`), App[$.FILENAME], [[6, 16]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var a;
	var $$promises = $.run([async () => a = (await $.track_reactivity_loss(Promise.resolve(1)))()]);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, {}, ($$anchor) => {
		let b;
		var promises = $.run([async () => b = $.tag((await $.save($.async_derived(async () => (await $.save(Promise.resolve(2)))())))(), "b")]);
		var fragment_1 = $.comment();
		var node_1 = $.first_child(fragment_1);
		$.async(node_1, [promises[0], $$promises[0]], void 0, (node_1) => {
			var consequent = ($$anchor) => {
				var p = root();
				$.append($$anchor, p);
			};
			$.add_svelte_meta(() => $.if(node_1, ($$render) => {
				if ($.get(b) + a > 0) $$render(consequent);
			}), "if", App, 6, 1);
		});
		$.append($$anchor, fragment_1);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
