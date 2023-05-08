import { BoilingSession } from "seba-rust";

let boiling_session = BoilingSession.new();

[
    "#mass-input",
    "#boiling-temperature-input",
    "#start-temperature-input",
].forEach(selector => {
    document.querySelector(selector).addEventListener("input", () => boiling_session.update_display());
});

setInterval(() => {
    boiling_session.update_display();
}, 100);

// init();
