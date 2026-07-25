import "svelte/internal/flags/async";
App[$.FILENAME] = "src/lib/Widget.svelte";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var first, second;
	var $$promises = $.run([async () => {
		var $$d = await $.async_derived(async () => (await $.track_reactivity_loss($$props.p))(), "[$derived object]", "src/​lib/​Widget.svelte:3:27");
		first = $.tag($.derived(() => $.get($$d).first), "first");
		second = $.tag($.derived(() => $.get($$d).second), "second");
	}]);
	var $$exports = { ...$.legacy_api() };
	var p_1 = root();
	var text = $.child(p_1);
	$.reset(p_1);
	$.template_effect(() => $.set_text(text, `${$.get(first) ?? ""} ${$.get(second) ?? ""}`), void 0, void 0, [$$promises[0]]);
	$.append($$anchor, p_1);
	return $.pop($$exports);
}
