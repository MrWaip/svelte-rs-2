import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var loaded, name;
	var $$promises = $.run([async () => loaded = (await $.track_reactivity_loss(Promise.resolve(1)))(), () => void 0]);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${loaded ?? ""}${$$props.name ?? ""}`), void 0, void 0, [$$promises[0], $$promises[1]]);
	$.append($$anchor, p);
	return $.pop($$exports);
}
