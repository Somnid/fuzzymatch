import initWasm, { fuzzymatch } from "../../../../crate/dist/pkg-web/fuzzymatch.js";
import { titles } from "../data/titles.js";

customElements.define("app-root",
	class extends HTMLElement {
		static get observedAttributes(){
			return [];
		}
		constructor(){
			super();
			this.bind(this);
		}
		bind(element){
			element.attachEvents = element.attachEvents.bind(element);
			element.cacheDom = element.cacheDom.bind(element);
		}
		async connectedCallback(){
			this.cacheDom();
			this.attachEvents();
			this.fuzzymatchPromise = initWasm("/crate/dist/pkg-web/fuzzymatch_bg.wasm");
		}
		cacheDom(){
			this.dom = {
                input : document.querySelector("#input"),
                output : document.querySelector("#output")
            };
		}
		attachEvents(){
            this.dom.input.addEventListener("input", async (e) => {
				const _ = await this.fuzzymatchPromise;
                const out = fuzzymatch(titles, e.target.value, 0.5).map(x => x[1]);
                output.innerHTML = out.join("\n");
            });
		}
		attributeChangedCallback(name, oldValue, newValue){
			this[name] = newValue;
		}
	}
)