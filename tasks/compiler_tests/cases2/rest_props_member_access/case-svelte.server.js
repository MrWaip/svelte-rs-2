import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { id, $$slots, $$events, ...props } = $$props;
		const label = $.derived(() => props.label + "!");
		const style = $.derived(() => props.style);
		const color = $.derived(() => props.style.color);
		$$renderer.push(`<p>${$.escape(label())}</p> <span>${$.escape(props.title)}</span> <div>${$.escape(props.nested.deep.value)}</div>`);
	});
}
