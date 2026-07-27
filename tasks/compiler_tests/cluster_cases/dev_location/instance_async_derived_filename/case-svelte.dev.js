import "svelte/internal/flags/async";
App[$.FILENAME] = "src/lib/Widget.svelte";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var total;
	var $$promises = $.run([async () => total = await $.async_derived(async () => (await $.track_reactivity_loss($$props.p))(), "total", "src/​lib/​Widget.svelte:3:15")]);
	var $$exports = { ...$.legacy_api() };
	var p_1 = root();
	var text = $.child(p_1, true);
	$.reset(p_1);
	$.template_effect(() => $.set_text(text, $.get(total)), void 0, void 0, [$$promises[0]]);
	$.append($$anchor, p_1);
	return $.pop($$exports);
}
