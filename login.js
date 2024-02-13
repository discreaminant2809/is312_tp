"use strict";

const goToSignUpElem = document.querySelector("#go-to-sign-up")
const submitBtnElem = document.querySelector("#submit-btn");
const usernameElem = document.querySelector("#username");
const passwordElem = document.querySelector("#password");
const errorLogElem = document.querySelector("#error-log");

goToSignUpElem.addEventListener("click", () => {
    location.href = "sign-up.html";
});

submitBtnElem.addEventListener("click", e => {
    e.preventDefault();

    errorLogElem.innerHTML = "";

    if (!authenticate(usernameElem.value, passwordElem.value)) {
        createErrorLogMsgElem("Failed to login: wrong username or password");
        return;
    }

    login();
});

function createErrorLogMsgElem(msg) {
    const errorLogMsgElem = document.createElement("p");
    errorLogMsgElem.textContent = msg;
    errorLogMsgElem.classList.add("error-log-msg");
    errorLogElem.appendChild(errorLogMsgElem);
}

function authenticate(username, pwd) {
    const expectedPwd = users.get(username);
    return expectedPwd !== undefined && pwd === expectedPwd;
}

function login() {
    location.href = "home.html";
}