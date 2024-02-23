import { invoke } from "@tauri-apps/api/tauri";

class MemoryText {
	domTextContener: HTMLTextAreaElement;
	from: number = 512;
	to: number = 570;

	constructor(domTextContener: HTMLTextAreaElement) {
		this.domTextContener = domTextContener
	}

	async fillContent() {
		this.domTextContener.textContent = await invoke("send_memory_content",
			{from: this.from, to: this.to}
		);
	}
}


const button   = document.getElementById("Invoker") as HTMLButtonElement;
const ram_text = document.getElementById("RAM")     as HTMLTextAreaElement;

let ram = new MemoryText(ram_text);

button.addEventListener("click", () => {ram.fillContent()});
