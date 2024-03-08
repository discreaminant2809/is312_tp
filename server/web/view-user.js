"use strict";

const tabElems = document.querySelectorAll(`.tab`);
const tabContentElems = document.querySelectorAll(`.tab-content`);
const urPostsElem = document.querySelector(`#ur-posts`);
const draftsElem = document.querySelector(`#drafts`);
const createNewPostElem = document.querySelector(`#create-new-post`);
const changePwdTextElem = document.querySelector(`#change-pwd-text`);
const changePwdForm = document.querySelector(`#change-pwd-form`);
const changePwdReenterElem = document.querySelector(`#change-pwd-reenter`);
const changePwdNewPwd = document.querySelector(`#change-pwd-new-pwd`);
const changePwdCancelElem = document.querySelector(`#change-pwd-cancel`);

async function getPosts() {
    const res = await fetch(`./api/posts`, {
        method: `POST`,
        mode: 'same-origin',
        headers: {
            'Content-Type': `application/json`,
        },
    });
    if (!res.ok) {
        return null;
    }

    return await res.json();
}

const SELECTED_CLASS_NAME = `selected`;
tabElems.forEach((tabElem, i, tabElems) => {
    tabElem.onclick = () => {
        tabElems.forEach(tabElem => tabElem.classList.remove(SELECTED_CLASS_NAME));
        tabContentElems.forEach(tabContentElem => tabContentElem.hidden = true);
        tabElem.classList.add(SELECTED_CLASS_NAME);
        tabContentElems[i].hidden = false;
    };
});
tabElems[0].classList.add(SELECTED_CLASS_NAME);
tabContentElems[0].hidden = false;
getPosts().then(posts => {
    if (posts === null) {
        urPostsElem.innerHTML += `
            <p class="error-log-msg">Something went wrong. Please log in again</p>
        `;
        return;
    }

    posts.forEach(post => {
        function deltaToSummary(content) {
            const MAX_SHOW_LEN = 50;
            content = content.ops
                .filter(op => typeof op.insert === 'string') // Keep only insert operations with string content
                .map(op => op.insert) // Extract the string content
                .join('');

            return content.length > MAX_SHOW_LEN
                ? `${content.slice(0, 47)}...`
                : content;
        }

        if (post.dateNum === undefined) {
            draftsElem.innerHTML += `
                <article class="blog-post" data-id="${post.id}">
                    <h2 class="post-title">${post.title}</h2>
                    <p class="post-summary">
                        ${deltaToSummary(post.content)}
                    </p>
                    <div class="post-options">
                        <a href="#" class="post-option">Edit</a>
                    </div>
                </article>
            `;
        } else {
            urPostsElem.innerHTML += `
                <article class="blog-post" data-id="${post.id}">
                    <div class="post-header">
                        <h2 class="post-title">${post.title}</h2>
                        <p class="post-date">${new Date(post.dateNum)}</p>
                    </div>
                    <p class="post-summary">
                        ${deltaToSummary(post.content)}
                    </p>
                    <div class="post-options">
                        <a href="#" class="post-option">Edit</a>
                    </div>
                </article>
            `;
        }
    });
});

createNewPostElem.onclick = () => {
    location.href = `edit-post.html?kind=newpost`;
}

changePwdTextElem.onclick = () => {
    changePwdForm.hidden = false;
};

changePwdCancelElem.onclick = e => {
    e.preventDefault();

    changePwdForm.hidden = true;
    changePwdReenterElem.value = ``;
    changePwdNewPwd.value = ``;
};

