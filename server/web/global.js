"use strict";

const usernameDisplayElem = document.querySelector(`#username-display`);
const signOutLinkELem = document.querySelector(`#sign-out-link`);

addEventListener(`DOMContentLoaded`, async () => {
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
});

signOutLinkELem.onclick = async () => {
    await fetch(`./api/signout`, {
        method: `POST`,
        mode: 'same-origin',
        headers: {
            'Content-Type': `application/json`,
        },
    });
}

function deltaToSummary(content) {
    const MAX_SHOW_LEN = 300;
    content = content.ops
        .filter(op => typeof op.insert === 'string')
        .map(op => op.insert)
        .join('');

    return content.length > MAX_SHOW_LEN
        ? `${content.slice(0, 47)}...`
        : content;
}