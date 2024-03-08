"use strict";

const postTitleElem = document.querySelector(`#post-title`);
const doneBtnElem = document.querySelector(`#done-btn`);
const deleteBtnElem = document.querySelector(`#delete-btn`);

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
    // new, draft, published

    constructor(kind, postId) {
        this.kind = kind;
        this.postId = postId;
    }
}

function addPublishButton() {
    const moreButtonsElem = document.querySelector(`#more-buttons`);
    // TODO: publish functionality
    moreButtonsElem.innerHTML += `
        <button type="submit" class="prevent-select primary-button">Publish</button>
    `;
}

onload = async () => {
    const searchParams = new URL(location.href).searchParams;
    const kind = searchParams.get("kind");
    const postId = searchParams;

    if (kind === `new`) {
        handler.kind === kind;
        addPublishButton();
        return;
    }

    const fetchUrl = `./api/editpost/requestedit/${searchParams.get("postid")}`;
    const res = await fetch(fetchUrl, {
        method: `POST`,
        mode: 'same-origin',
        headers: {
            'Content-Type': `application/json`,
        },
    });
    const post = await res.json();

    postTitleElem.value = post.title;
    postContentEditor.setContents(post.content);
    handler.kind === kind;
    if (kind === `draft`) {
        addPublishButton();
    }
}

doneBtnElem.onclick = e => {
    e.preventDefault();
};

deleteBtnElem.onclick = e => {
    e.preventDefault();
};