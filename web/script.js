"use strict";
import init, {solve_puzzle} from "./pkg/wasm.js";

const inputs = [];

function createInputCell(r, c) {
    const input = document.createElement("input");
    input.type = "text";
    input.classList.add("input-cell");
    input.value = "";
    input.pattern = "[0-9]*";

    const container = document.createElement("div");
    container.classList.add("input-container");
    if (c % 3 === 2 && c !== 8) {
        container.classList.add("column-separator");
    }

    if (r % 3 === 2 && r !== 8) {
        container.classList.add("row-separator");
    }
    container.appendChild(input);
    return container;
}

function handleSolve(e) {
    e.preventDefault();
    const puzzle = inputs.map((input, index) => (input.children[0].value || "0") + ((index + 1) % 9 === 0 ? "\n" : "")).join("");
    const rawResult = solve_puzzle(puzzle);
    const values = rawResult.split("");
    for (let i = 0; i < values.length; i++) {
        const input = inputs[i].children[0];
        if (input.value === "") {
            input.value = values[i];
            input.classList.add("solved");

        }
    }
}

function handleReset() {
    inputs.forEach(input => {
        input.children[0].classList.remove("solved");
    });
}

function initializeInputGrid() {
    const inputGrid = document.getElementById("input-grid");
    for (let r = 0; r < 9; r++) {
        for (let c = 0; c < 9; c++) {
            const input = createInputCell(r, c);
            inputs.push(input);
            inputGrid.appendChild(input);
        }
    }
}

init().then(() => {
    initializeInputGrid();

    const entryForm = document.getElementById("entry-form");
    const resetButton = document.getElementById("reset-button");

    entryForm.addEventListener("submit", handleSolve);
    resetButton.addEventListener("click", handleReset);
});
