import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	const $s3 = () => $.store_get(s3(), "$s3", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let n1 = $.prop($$props, "n1", 8);
	let n2 = $.prop($$props, "n2", 8);
	let s3 = $.prop($$props, "a6", 8);
	$.next();
	var text = $.text();
	$.template_effect(() => $.set_text(text, `$s3=${$s3() ?? ""}
${n1() ?? ""}${n2() ?? ""}`));
	$.append($$anchor, text);
	$$cleanup();
}
