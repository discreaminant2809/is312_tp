"use strict";

const usernameDisplayElem = document.querySelector(`#username-display`);

onload = async () => {
    function setError() {
        usernameDisplayElem.textContent = `Login to get started!`;
    }

    try {
        const res = await fetch(`http://localhost:3000/api/user`, {
            method: `GET`,
            mode: 'same-origin',
            headers: {
                'Content-Type': `application/json`,
            },
        });

        if (!res.ok) {
            setError();
            return;
        }

        usernameDisplayElem.textContent = `Welcome, ${await res.text()}!`
    } catch {
        setError();
    }
};
