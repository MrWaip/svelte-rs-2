import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let url = "/api";
	var data, meta;
	var $$promises = $.run([async () => {
		var $$d = await $.async_derived(async () => (await $.track_reactivity_loss(fetch(url).then((r) => r.json())))(), "[$derived object]", "(unknown):3:22");
		data = $.tag($.derived(() => $.get($$d).data), "data");
		meta = $.tag($.derived(() => $.get($$d).meta), "meta");
	}]);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(data) ?? ""}-${$.get(meta) ?? ""}`), void 0, void 0, [$$promises[0]]);
	$.append($$anchor, p);
	return $.pop($$exports);
}
