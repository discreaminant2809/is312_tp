"use strict";

const goToLogin = document.querySelector("#go-to-login");
const submitBtnElem = document.querySelector("#submit-btn");
const usernameElem = document.querySelector("#username");
const passwordElem = document.querySelector("#password");
const confirmPasswordElem = document.querySelector("#confirm-password");
const errorLogElem = document.querySelector("#error-log");
const showHideButtons = document.querySelectorAll(`.show-hide-btn`);

[passwordElem, confirmPasswordElem].forEach((elem, i) => {
    showHideButtons[i].onclick = () => {
        if (elem.type === `password`) {
            elem.type = `input`;
            showHideButtons[i].textContent = `Hide`;
        } else {
            elem.type = `password`;
            showHideButtons[i].textContent = `Show`;
        }
    }
});

goToLogin.onclick = () =>{
    location.href = "login.html";
};

submitBtnElem.onclick = async e => {
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

    const result = await register(usernameElem.value, passwordElem.value);
    if (result.failed) {
        createErrorLogMsgElem(result.msg);
        return;
    }

    alert(result.msg);
    goToLogin.onclick();
};

function createErrorLogMsgElem(msg) {
    const errorLogMsgElem = document.createElement("p");
    errorLogMsgElem.textContent = msg;
    errorLogMsgElem.classList.add("error-log-msg");
    errorLogElem.appendChild(errorLogMsgElem);
}

async function register(username, pwd) {
    const res = await fetch(`./api/signup`, {
        method: `POST`,
        mode: 'same-origin',
        headers: {
            'Content-Type': `application/json`,
        },
        body: JSON.stringify({username, pwd}),
    });

    return {failed: !res.ok, msg: await res.text()};
}