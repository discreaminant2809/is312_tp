"use strict";

const usernameDisplayElem = document.querySelector(`#username-display`);
const signOutLinkELem = document.querySelector(`#sign-out-link`);

onload = async () => {
    const res = await fetch(`http://localhost:3000/api/user`, {
        method: `GET`,
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
    const res = await fetch(`http://localhost:3000/api/signout`, {
        method: `GET`,
        mode: 'same-origin',
        headers: {
            'Content-Type': `application/json`,
        },
    });
}