"use strict"

const postHeaderElem = document.querySelector(`#post-header`);
const postAuthorElem = document.querySelector(`#post-author`);
const postDateElem = document.querySelector(`#post-date`);
const postContentElem = document.querySelector(`#post-content`);

addEventListener("DOMContentLoaded", async () => {
    const postId = Number(new URL(location.href).searchParams.get("postid"));
    if (Number.isNaN(postId)) {
        alert(`Invalid parameters! Post cannot be loaded`);
        return;
    }

    const res = await fetch(`./api/viewpost?postid=${postId}`, {
        method: `GET`,
        mode: 'same-origin',
        headers: {
            'Content-Type': `application/json`,
        },
    });

    if (!res.ok) {
        alert(`No such post! Post cannot be loaded`);
        return;
    }

    const post = await res.json();

    postHeaderElem.textContent = post.title;
    postAuthorElem.textContent = post.author;
    postDateElem.textContent = new Date(post.dateNum).toDateString();
    const postContent = new Quill(postContentElem, {
        modules: {
            toolbar: false,
        },
        readOnly: true,
        theme: `snow`,
    });
    postContent.setContents(post.content);
});