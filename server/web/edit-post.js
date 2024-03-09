"use strict";

const postTitleElem = document.querySelector(`#post-title`);
const errorLogElem = document.querySelector(`#error-log`);
const saveNotificationElem = document.querySelector(`#save-notification`);

const toolbarOptions = [
    [{'font': []}],
    ['bold', 'italic', 'underline', 'strike'],
    ['blockquote', 'code-block'],
    ['link', 'image'],

    [{'header': 1}, {'header': 2}],
    [{'list': 'ordered'}, {'list': 'bullet'}],
    [{'script': 'sub'}, {'script': 'super'}],
    [{'indent': '+1'}, {'indent': '-1'}],
    [{'direction': 'rtl'}],

    [{'size': ['small', false, 'large', 'huge']}],
    [{'header': [1, 2, 3, false]}],

    [{'color': []}, {'background': []}],
    [{'align': []}],

    ['clean'] // clear-all-formats button
];

const postContentEditor = new Quill('#post-content', {
    modules: {
        toolbar: toolbarOptions
    },
    theme: 'snow'
});

const handler = {

    postId: -1,
    canSaveFlag: true,
    canSaveNow() {
        if (!this.canSaveFlag) {
            return false;
        }

        this.canSaveFlag = false;
        setTimeout(() => this.canSaveFlag = true, 500);
        return true;
    },
    hasUnsavedChange: false,
}

postContentEditor.on(`text-change`, () => {
    handler.hasUnsavedChange = true;
    saveNotificationElem.textContent = ``;
});

function addButtons(kind) {
    function createButton(name, type, listener) {
        const buttonElem = document.createElement(`button`);
        buttonElem.classList.add(`prevent-select`, type);
        buttonElem.type = `submit`;
        buttonElem.textContent = name;
        buttonElem.onclick = e => {
            e.preventDefault();
            listener(buttonElem, e);
        };
        return buttonElem;
    }

    const submitButtonsElem = document.querySelector(`#submit-buttons`);

    async function publishNew() {
        const post = {
            title: postTitleElem.value,
            content: postContentEditor.getContents(),
        };

        const res = await fetch(`./api/editpost/publish`, {
            method: `POST`,
            mode: 'same-origin',
            headers: {
                'Content-Type': `application/json`,
            },
            body: JSON.stringify(post),
        });
        if (!res.ok) {
            errorLogElem.textContent = `Failed to publish post. Something went wrong`;
            return;
        }

        location.href = `view-user.html`;
    }

    async function publish() {
        const res = await fetch(`./api/editpost/publish/${handler.postId}`, {
            method: `POST`,
            mode: 'same-origin',
            headers: {
                'Content-Type': `application/json`,
            },
        });
        if (!res.ok) {
            errorLogElem.textContent = `Failed to publish post. Something went wrong`;
            return;
        }

        location.href = `view-user.html`;
    }

    async function draft() {
        const post = {
            title: postTitleElem.value,
            content: postContentEditor.getContents(),
        };

        const res = await fetch(`./api/editpost/newpost`, {
            method: `POST`,
            mode: 'same-origin',
            headers: {
                'Content-Type': `application/json`,
            },
            body: JSON.stringify(post),
        });
        if (!res.ok) {
            errorLogElem.textContent = `Failed to save post as draft. Something went wrong`;
            return;
        }

        location.href = `view-user.html`;
    }

    async function save() {
        if (!(handler.hasUnsavedChange && handler.canSaveNow())) {
            return;
        }

        const post = {
            title: postTitleElem.value,
            content: postContentEditor.getContents(),
        };

        const res = await fetch(`./api/editpost/savechange/${handler.postId}`, {
            method: `POST`,
            mode: 'same-origin',
            headers: {
                'Content-Type': `application/json`,
            },
            body: JSON.stringify(post),
        });
        if (!res.ok) {
            errorLogElem.textContent = `Failed to save post. Something went wrong`;
            return;
        }

        handler.hasUnsavedChange = false;
        saveNotificationElem.textContent = `Saved ☑`;
    }
    onkeydown = async e => {
        if (e.ctrlKey && e.key.toLowerCase() === `c`) {
            await save();
        }
    };

    async function deleteAsDraftOrPublished() {
        const res = await fetch(`./api/editpost/delete/${handler.postId}`, {
            method: `DELETE`,
            mode: 'same-origin',
            headers: {
                'Content-Type': `application/json`,
            },
        });
        if (!res.ok) {
            errorLogElem.textContent = `Failed to delete post. Something went wrong`;
            return;
        }

        location.href = `view-user.html`;
    }

    switch (kind) {
        case `new`:
            submitButtonsElem.appendChild(
                createButton(`Publish`, `primary-button`, publishNew)
            );
            submitButtonsElem.appendChild(
                createButton(`Draft`, `primary-button`, draft)
            );
            submitButtonsElem.appendChild(
                createButton(`Delete`, `delete-button`, () => {
                    location.href = `view-user.html`;
                })
            );
            break;
        case `draft`:
            submitButtonsElem.appendChild(
                createButton(`Publish`, `primary-button`, publish)
            );
            submitButtonsElem.appendChild(
                createButton(`Save`, `primary-button`, save)
            );
            submitButtonsElem.appendChild(
                createButton(`Delete`, `delete-button`, deleteAsDraftOrPublished)
            );
            break;
        case `published`:
            submitButtonsElem.appendChild(
                createButton(`Save`, `primary-button`, save)
            );
            submitButtonsElem.appendChild(
                createButton(`Delete`, `delete-button`, deleteAsDraftOrPublished)
            );
    }
}

onload = async () => {
    const searchParams = new URL(location.href).searchParams;
    // Possible value: new, draft, published
    const kind = searchParams.get(`kind`);

    if (kind === `new`) {
        handler.kind = kind;
        addButtons(kind);
        return;
    }

    if (![`draft`, `published`].includes(kind)) {
        errorLogElem.textContent = `Something went wrong`;
        return;
    }

    handler.postId = Number(searchParams.get(`postid`));
    if (Number.isNaN(handler.postId)) {
        errorLogElem.textContent = `Something went wrong`;
        return;
    }

    const fetchUrl = `./api/editpost/requestedit/${handler.postId}`;
    const res = await fetch(fetchUrl, {
        method: `POST`,
        mode: 'same-origin',
        headers: {
            'Content-Type': `application/json`,
        },
    });
    if (!res.ok) {
        errorLogElem.textContent = `Cannot request editing. Something went wrong`;
        return;
    }

    const post = await res.json();

    postTitleElem.value = post.title;
    postContentEditor.setContents(post.content);
    addButtons(kind);
}