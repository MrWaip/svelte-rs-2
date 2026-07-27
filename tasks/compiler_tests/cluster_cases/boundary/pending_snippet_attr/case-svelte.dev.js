import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const pending = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	$.next();
	var text = $.text("loading");
	$.append($$anchor, text);
});
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.boundary(node, { get pending() {
		return pending;
	} }, ($$anchor) => {
		let data;
		var promises = $.run([async () => data = $.tag((await $.save($.async_derived(async () => (await $.save(Promise.resolve("d")))())))(), "data")]);
		$.next();
		var text_1 = $.text();
		$.template_effect(() => $.set_text(text_1, $.get(data)), void 0, void 0, [promises[0]]);
		$.append($$anchor, text_1);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
