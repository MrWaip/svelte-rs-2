import * as $ from "svelte/internal/client";
var root_1 = $.from_html(`<p> </p>`);
export default function App($$anchor) {
	const view = ($$anchor, $$arg0) => {
		let value = () => $$arg0?.()[key()];
		let rest = () => $.exclude_from_object($$arg0?.(), [String(key())]);
		var p = root_1();
		var text = $.child(p);
		$.reset(p);
		$.template_effect(() => $.set_text(text, `${value() ?? ""} ${rest().extra ?? ""}`));
		$.append($$anchor, p);
	};
	let data = $.proxy({
		label: "world",
		extra: "ok"
	});
	function key() {
		return "label";
	}
	view($$anchor, () => data);
}
