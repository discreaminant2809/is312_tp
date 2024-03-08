"use strict";

const usernameDisplayElem = document.querySelector(`#username-display`);
const signOutLinkELem = document.querySelector(`#sign-out-link`);

onload = async () => {
    const res = await fetch(`./api/user`, {
        method: `POST`,
        mode: 'same-origin',
        headers: {
            'Content-Type': `application/json`,
        },
    });

    if (!res.ok) {
        location.href = "login.html";
        return;
    }

    usernameDisplayElem.textContent = `Welcome, ${await res.text()}!`;
};

signOutLinkELem.onclick = async () => {
    await fetch(`./api/signout`, {
        method: `POST`,
        mode: 'same-origin',
        headers: {
            'Content-Type': `application/json`,
        },
    });
}