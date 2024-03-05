"use strict";

const goToSignUpElem = document.querySelector("#go-to-sign-up")
const submitBtnElem = document.querySelector("#submit-btn");
const usernameElem = document.querySelector("#username");
const passwordElem = document.querySelector("#password");
const errorLogElem = document.querySelector("#error-log");
const showHideButton = document.querySelector(`.show-hide-btn`);

showHideButton.onclick = () => {
    if (passwordElem.type === `password`) {
        passwordElem.type = `input`;
        showHideButton.textContent = `Hide`;
    } else {
        passwordElem.type = `password`;
        showHideButton.textContent = `Show`;
    }
}

goToSignUpElem.onclick = () => {
    location.href = "sign-up.html";
};

submitBtnElem.onclick = async e => {
    e.preventDefault();

    errorLogElem.innerHTML = "";

    const result = await authenticate(usernameElem.value, passwordElem.value);
    if (result.failed) {
        createErrorLogMsgElem(result.msg);
        return;
    }

    alert(result.msg);
    login();
};

function createErrorLogMsgElem(msg) {
    const errorLogMsgElem = document.createElement("p");
    errorLogMsgElem.textContent = msg;
    errorLogMsgElem.classList.add("error-log-msg");
    errorLogElem.appendChild(errorLogMsgElem);
}

async function authenticate(username, pwd) {
    try {
        const res = await fetch(`http://127.0.0.1:3000/api/login`, {
            method: `POST`,
            headers: {
                'Content-Type': `application/json`,
            },
            body: JSON.stringify({username, pwd}),
        });

        return {failed: !res.ok, msg: await res.text()};
    } catch (e) {
        return {failed: true, msg: `Something went wrong`};
    }
}

function login() {
    location.href = "home.html";
}