import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const content = $.wrap_snippet(App, function($$anchor, value = $.noop) {
	$.validate_snippet_args(...arguments);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, value()));
	$.append($$anchor, p);
});
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var data;
	var $$promises = $.run([async () => data = (await $.track_reactivity_loss(fetch("/api")))()]);
	var $$exports = { ...$.legacy_api() };
	$.async($$anchor, [$$promises[0]], void 0, ($$anchor) => {
		$.add_svelte_meta(() => content($$anchor, () => data), "render", App, 9, 0);
	});
	$.next();
	return $.pop($$exports);
}
