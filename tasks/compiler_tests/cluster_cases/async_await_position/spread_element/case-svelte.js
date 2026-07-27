import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> `, 1);
export default function App($$anchor) {
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve([value]);
	}
	function join(...args) {
		return args.length;
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var text = $.sibling(button);
	$.template_effect(($0, $1) => $.set_text(text, ` ${$0 ?? ""}
${$1 ?? ""}`), void 0, [async () => join(...(await $.save(delay($.get(x))))()), async () => join([...(await $.save(delay($.get(x))))()])]);
	$.delegated("click", button, () => $.update(x));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
