import * as $ from "svelte/internal/server";
function greeting($$renderer, { name, age }) {
	$$renderer.push(`<p>${$.escape(name)} is ${$.escape(age)}</p>`);
}
function withDefault($$renderer, { label = "default" }) {
	$$renderer.push(`<span>${$.escape(label)}</span>`);
}
function withRest($$renderer, { id, ...rest }) {
	$$renderer.push(`<div>${$.escape(id)}</div>`);
}
export default function App($$renderer) {
	let data = {
		name: "world",
		age: 25
	};
	greeting($$renderer, data);
	$$renderer.push(`<!----> `);
	withDefault($$renderer, {});
	$$renderer.push(`<!----> `);
	withRest($$renderer, {
		id: 1,
		extra: true
	});
	$$renderer.push(`<!---->`);
}
