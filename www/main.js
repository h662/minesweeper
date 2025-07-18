import init, {Minesweeper} from "./pkg/minesweeper.js";

async function run() {
    await init();
    const game = new Minesweeper();

    const btnReveal = document.getElementById("btn-reveal");
    const btnFlag = document.getElementById("btn-flag");
    const btnQuestion = document.getElementById("btn-question");
    const modeDisplay = document.getElementById("mode-display");

    btnReveal.addEventListener("click", () => {
        game.set_mode("reveal");
        modeDisplay.textContent = "클릭";
    });
    btnFlag.addEventListener("click", () => {
        game.set_mode("flag");
        modeDisplay.textContent = "지뢰표시";
    });
    btnQuestion.addEventListener("click", () => {
        game.set_mode("question");
        modeDisplay.textContent = "물음표";
    });

    const overlay = document.getElementById("gameover-overlay");
    const btnRestartPopup = document.getElementById("btn-restart-popup");
    btnRestartPopup.addEventListener("click", () => {
        overlay.style.display = "none";
        console.log(overlay.style.display)
        game.restart();           // WASM restart 호출
    });

    function frame() {
        game.update();
        game.render();

        if (game.is_game_over()) {
            if (game.is_game_won()) {
                messageEl.textContent = "🎉 You Win! 🎉";
            } else {
                messageEl.textContent = "💥 Game Over 💥";
            }

            overlay.style.display = "flex";
        }

        requestAnimationFrame(frame);
    }

    frame();
}

run();