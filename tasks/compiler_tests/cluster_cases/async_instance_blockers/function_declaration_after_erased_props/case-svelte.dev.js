import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function f() {
		return 1;
	}
	var a, b;
	var $$promises = $.run([async () => a = (await $.track_reactivity_loss($$props.p))(), () => b = 2]);
	var $$exports = { ...$.legacy_api() };
	$.next();
	var text = $.text();
	$.template_effect(($0) => $.set_text(text, `${a ?? ""}2${$0 ?? ""}`), [() => f()], void 0, [$$promises[0], $$promises[1]]);
	$.append($$anchor, text);
	return $.pop($$exports);
}
