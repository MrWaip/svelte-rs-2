import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span> </span>`), App[$.FILENAME], [[9, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const row = $.wrap_snippet(App, function($$anchor) {
		$.validate_snippet_args(...arguments);
		let doubled;
		var promises_1 = $.run([() => promises[0].promise, () => doubled = $.tag($.derived(() => $.get(number) * 2), "doubled")]);
		var span = root();
		var text = $.child(span, true);
		$.reset(span);
		$.template_effect(() => $.set_text(text, $.get(doubled)), void 0, void 0, [promises_1[1]]);
		$.append($$anchor, span);
	});
	let n = 1;
	var $$exports = { ...$.legacy_api() };
	let number;
	var promises = $.run([async () => number = await $.async_derived(async () => (await $.track_reactivity_loss(Promise.resolve(n)))(), "number", "(unknown):5:14")]);
	$.add_svelte_meta(() => row($$anchor), "render", App, 12, 0);
	return $.pop($$exports);
}
