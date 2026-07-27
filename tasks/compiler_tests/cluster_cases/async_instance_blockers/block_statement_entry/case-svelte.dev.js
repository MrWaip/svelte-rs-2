import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var data, y;
	var $$promises = $.run([async () => data = (await $.track_reactivity_loss(fetch("/a")))(), () => {
		{
			console.log(1);
			console.log(2);
		}
		y = 1;
	}]);
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, `${data ?? ""}1`), void 0, void 0, [$$promises[0], $$promises[1]]);
	$.append($$anchor, text);
	return $.pop($$exports);
}
