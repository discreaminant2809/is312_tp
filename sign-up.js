"use strict";

const goToLogin = document.querySelector("#go-to-login");
const submitBtnElem = document.querySelector("#submit-btn");
const usernameElem = document.querySelector("#username");
const passwordElem = document.querySelector("#password");
const confirmPasswordElem = document.querySelector("#confirm-password");
const errorLogElem = document.querySelector("#error-log");

goToLogin.addEventListener("click", () =>{
    location.href = "login.html";
});

submitBtnElem.addEventListener("click", e => {
    e.preventDefault();

    errorLogElem.innerHTML = "";
    let valid = true;
    usernameElem.value = usernameElem.value.trimEnd();

    if (usernameElem.value.length === 0) {
        createErrorLogMsgElem("Username cannot be empty")
        valid = false;
    }

    if (passwordElem.value.length === 0) {
        createErrorLogMsgElem("Password cannot be empty")
        valid = false;
    }

    if (confirmPasswordElem.value.length === 0) {
        createErrorLogMsgElem("Confirm password cannot be empty")
        valid = false;
    }

    if (passwordElem.value !== confirmPasswordElem.value) {
        createErrorLogMsgElem("Password and confirm password do not match");
        valid = false;
    }

    if (!valid) {
        return;
    }

    if (!register(usernameElem.value, passwordElem.value)) {
        createErrorLogMsgElem("Failed to sign up: such user already exists");
        return;
    }

    alert("Registration successful!");
});

function createErrorLogMsgElem(msg) {
    const errorLogMsgElem = document.createElement("p");
    errorLogMsgElem.textContent = msg;
    errorLogMsgElem.classList.add("error-log-msg");
    errorLogElem.appendChild(errorLogMsgElem);
}

function register(username, pwd) {
    if (users.has(username)) {
        return false;
    }

    users.set(username, pwd);
    return true;
}