import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var name, age;
	var $$promises = $.run([async () => {
		var $$d = await $.async_derived(async () => (await $.track_reactivity_loss(fetch("/api")))(), "[$derived object]");
		name = $.derived(() => $.get($$d).name);
		age = $.derived(() => $.get($$d).age);
	}]);
	var $$exports = { ...$.legacy_api() };
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${$.get(name) ?? ""} ${$.get(age) ?? ""}`), void 0, void 0, [$$promises[0]]);
	$.append($$anchor, p);
	return $.pop($$exports);
}
