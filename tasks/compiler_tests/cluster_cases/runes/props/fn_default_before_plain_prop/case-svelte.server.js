import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { getText = (item) => String(item), getUrl } = $$props;
		$$renderer.push(`<span>${$.escape(getText(1))}${$.escape(getUrl)}</span>`);
	});
}
