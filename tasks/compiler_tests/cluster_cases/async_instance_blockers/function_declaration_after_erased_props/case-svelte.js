import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	function f() {
		return 1;
	}
	var a, b;
	var $$promises = $.run([async () => a = await $$props.p, () => b = 2]);
	$.next();
	var text = $.text();
	$.template_effect(($0) => $.set_text(text, `${a ?? ""}2${$0 ?? ""}`), [() => f()], void 0, [$$promises[0], $$promises[1]]);
	$.append($$anchor, text);
}
