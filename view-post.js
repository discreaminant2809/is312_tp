"use strict"

const postHHeaderElem = document.querySelector(`#post-header`);
const postAuthorElem = document.querySelector(`#post-author`);
const postDateElem = document.querySelector(`#post-date`);
const postContentElem = document.querySelector(`#post-content`);

onload = async () => {
    const post = await loadPost();
    postHHeaderElem.textContent = post.header;
    postAuthorElem.textContent = post.author;
    postDateElem.textContent = post.date.toDateString();
    postContentElem.innerHTML = post.content;
};

async function loadPost() {
    return testPost;
}

const testPost = {
    header: `Lorem Ipsum`,
    author: `Jackson`,
    date: new Date(2024, 11, 24),
    content: `
        <p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore
            magna aliqua. Ullamcorper a lacus vestibulum sed. Scelerisque eleifend donec pretium vulputate sapien. Eu
            lobortis elementum nibh tellus molestie. Quis varius quam quisque id diam. Aliquam sem et tortor consequat
            id porta nibh venenatis cras.</p>
        <p>Ut ornare lectus sit amet est. Ligula ullamcorper malesuada proin libero nunc.</p>
    `
};