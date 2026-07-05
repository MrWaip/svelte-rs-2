import * as $ from "svelte/internal/server";
import { f, g } from "./x";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { url = "", label = "" } = $$props;
		$$renderer.push(`<a${$.attr("href", f(url))}>${$.escape(g(label))}</a>`);
	});
}
